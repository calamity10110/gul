// IR Generator using Cranelift

use crate::ast::{Program, Statement, Expression, ExpressionKind, BinaryOperator, Type as AstType, Pattern, Param, OwnershipMode};
use anyhow::{anyhow, Result};
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module, FuncId, DataId, DataDescription};
use cranelift::codegen;
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use std::collections::HashMap;

pub struct IrGenerator {
    builder_context: FunctionBuilderContext,
    data_description: DataDescription,
}

struct LoopContext {
    header_block: Block,
    exit_block: Block,
}

struct GenContext<'a, 'b> {
    builder: &'a mut FunctionBuilder<'b>,
    module: &'a mut ObjectModule,
    variables: &'a mut HashMap<String, Variable>,
    var_counter: &'a mut usize,
    loop_stack: &'a mut Vec<LoopContext>,
    printf_id: FuncId,

    malloc_id: FuncId,
    realloc_id: FuncId,
    memmove_id: FuncId,
    gul_concat_id: FuncId,
    gul_int_to_str_id: FuncId,
    gul_float_to_str_id: FuncId,
    gul_print_float_id: FuncId,
    strcmp_id: FuncId,
    format_int_id: DataId,
    format_str_id: DataId,
    format_true_id: DataId,
    format_false_id: DataId,
    string_literals: &'a mut HashMap<String, DataId>,
    data_description: &'a mut DataDescription,
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            builder_context: FunctionBuilderContext::new(),
            data_description: DataDescription::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> Result<Vec<u8>> {
        let isa_builder = cranelift::codegen::isa::lookup(Triple::host())
            .map_err(|e| anyhow!("Failed to lookup ISA: {}", e))?;
        
        let flags = settings::Flags::new(settings::builder());
        let isa = isa_builder.finish(flags)
            .map_err(|e| anyhow!("Failed to create ISA: {}", e))?;
        
        let builder = ObjectBuilder::new(
            isa,
            "gul_program",
            cranelift_module::default_libcall_names(),
        ).map_err(|e| anyhow!("Failed to create object builder: {}", e))?;
        
        let mut module = ObjectModule::new(builder);
        
        // Declare printf (extern)
        let mut sig_printf = module.make_signature();
        sig_printf.params.push(AbiParam::new(types::I64)); 
        sig_printf.params.push(AbiParam::new(types::I64)); 
        sig_printf.returns.push(AbiParam::new(types::I32));
    let printf_id = module.declare_function("printf", Linkage::Import, &sig_printf)
        .map_err(|e| anyhow!("Failed to declare printf: {}", e))?;

        // Declare malloc (extern)
        let mut sig_malloc = module.make_signature();
        sig_malloc.params.push(AbiParam::new(types::I64));
        sig_malloc.returns.push(AbiParam::new(types::I64));
        let malloc_id = module.declare_function("malloc", Linkage::Import, &sig_malloc)
             .map_err(|e| anyhow!("Failed to declare malloc: {}", e))?;

        // Declare realloc (extern)
        let mut sig_realloc = module.make_signature();
        sig_realloc.params.push(AbiParam::new(types::I64));
        sig_realloc.params.push(AbiParam::new(types::I64));
        sig_realloc.returns.push(AbiParam::new(types::I64));
        let realloc_id = module.declare_function("realloc", Linkage::Import, &sig_realloc)
             .map_err(|e| anyhow!("Failed to declare realloc: {}", e))?;

        // Declare memmove
        let mut sig_memmove = module.make_signature();
        sig_memmove.params.push(AbiParam::new(types::I64));
        sig_memmove.params.push(AbiParam::new(types::I64));
        sig_memmove.params.push(AbiParam::new(types::I64));
        sig_memmove.returns.push(AbiParam::new(types::I64));
        let memmove_id = module.declare_function("memmove", Linkage::Import, &sig_memmove)
             .map_err(|e| anyhow!("Failed to declare memmove: {}", e))?;

        // Declare gul_string_concat
        let mut sig_concat = module.make_signature();
        sig_concat.params.push(AbiParam::new(types::I64)); 
        sig_concat.params.push(AbiParam::new(types::I64));
        sig_concat.returns.push(AbiParam::new(types::I64));
        let gul_concat_id = module.declare_function("gul_string_concat", Linkage::Import, &sig_concat)
            .map_err(|e| anyhow!("Failed to declare concat: {}", e))?;

        // Declare gul_int_to_string
        let mut sig_i2s = module.make_signature();
        sig_i2s.params.push(AbiParam::new(types::I64));
        sig_i2s.returns.push(AbiParam::new(types::I64));
        let gul_int_to_str_id = module.declare_function("gul_int_to_string", Linkage::Import, &sig_i2s)
            .map_err(|e| anyhow!("Failed to declare i2s: {}", e))?;

    // Declare gul_float_to_string
    let mut sig_f2s = module.make_signature();
    sig_f2s.params.push(AbiParam::new(types::F64));
    sig_f2s.returns.push(AbiParam::new(types::I64)); // char*
    let gul_float_to_str_id = module.declare_function("gul_float_to_string", Linkage::Import, &sig_f2s)
            .map_err(|e| anyhow!("Failed to declare f2s: {}", e))?;

    // Declare gul_print_float
    let mut sig_pf = module.make_signature();
    sig_pf.params.push(AbiParam::new(types::F64));
    sig_pf.returns.push(AbiParam::new(types::I32));
    let gul_print_float_id = module.declare_function("gul_print_float", Linkage::Import, &sig_pf)
            .map_err(|e| anyhow!("Failed to declare gul_print_float: {}", e))?;

        // Declare strcmp
        let mut sig_strcmp = module.make_signature();
        sig_strcmp.params.push(AbiParam::new(types::I64));
        sig_strcmp.params.push(AbiParam::new(types::I64));
        sig_strcmp.returns.push(AbiParam::new(types::I32)); // Returns i32
        let strcmp_id = module.declare_function("strcmp", Linkage::Import, &sig_strcmp)
             .map_err(|e| anyhow!("Failed to declare strcmp: {}", e))?;

        // Declare format string data for INT "%ld\n\0"
        self.data_description.define(b"%ld\n\0".to_vec().into_boxed_slice());
        let format_int_id = module.declare_data("fmt_int", Linkage::Local, true, false)
             .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_int_id, &self.data_description)
             .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        // Declare format string data for STR "%s\n\0"
        self.data_description.define(b"%s\n\0".to_vec().into_boxed_slice());
        let format_str_id = module.declare_data("fmt_str", Linkage::Local, true, false)
             .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_str_id, &self.data_description)
             .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        // Declare format string data for FLOAT "%f\n\0"
        self.data_description.define(b"%f\n\0".to_vec().into_boxed_slice());
        let format_float_id = module.declare_data("fmt_float", Linkage::Local, true, false)
             .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_float_id, &self.data_description)
             .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        // Declare "true\0"
        self.data_description.define(b"true\0".to_vec().into_boxed_slice());
        let format_true_id = module.declare_data("str_true", Linkage::Local, true, false)
             .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_true_id, &self.data_description)
             .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        // Declare "false\0"
        self.data_description.define(b"false\0".to_vec().into_boxed_slice());
        let format_false_id = module.declare_data("str_false", Linkage::Local, true, false)
             .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_false_id, &self.data_description)
             .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        let mut ctx = codegen::Context::new();
        let mut string_literals = HashMap::new(); // Cache string literals
        
        for stmt in &program.statements {
            if let Statement::Function { name, params, body } = stmt {
                self.generate_function(
                    name, params, body, 
                    &mut module, &mut ctx,
                    printf_id, malloc_id, realloc_id, memmove_id, gul_concat_id, gul_int_to_str_id, gul_float_to_str_id, gul_print_float_id, strcmp_id,
                    format_int_id, format_str_id, format_true_id, format_false_id,
                    &mut string_literals
                )?;
            }
        }
        
        let product = module.finish();
        Ok(product.emit().map_err(|e| anyhow!("Failed to emit object: {}", e))?)
    }

    fn generate_function(
        &mut self,
        name: &str,
        params: &[Param],
        body: &[Statement],
        module: &mut ObjectModule,
        ctx: &mut codegen::Context,
        printf_id: FuncId,
        malloc_id: FuncId,
        realloc_id: FuncId,
        memmove_id: FuncId,
        gul_concat_id: FuncId,
        gul_int_to_str_id: FuncId,
        gul_float_to_str_id: FuncId,
        gul_print_float_id: FuncId,
        strcmp_id: FuncId,
        format_int_id: DataId,
        format_str_id: DataId,
        format_true_id: DataId,
        format_false_id: DataId,
        string_literals: &mut HashMap<String, DataId>,
    ) -> Result<()> {
        let mut sig = module.make_signature();
        for param in params {
             let ty = match param.mode {
                 _ => if let AstType::Float = param.ty { types::F64 } else { types::I64 }
             };
             sig.params.push(AbiParam::new(ty));
        }
        sig.returns.push(AbiParam::new(types::I64));
        
        let func_id = module
            .declare_function(name, Linkage::Export, &sig)
            .map_err(|e| anyhow!("Failed to declare function: {}", e))?;
        
        ctx.func.signature = sig;
        
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        
        let mut loop_stack = Vec::new();
        let mut variables = HashMap::new();
        let mut var_counter = 0;
        
        for (i, param) in params.iter().enumerate() {
            let val = builder.block_params(entry_block)[i];
            let var = Variable::new(var_counter);
            var_counter += 1;
            
            let ty = match param.mode {
                 _ => if let AstType::Float = param.ty { types::F64 } else { types::I64 }
            };
            builder.declare_var(var, ty);
            builder.def_var(var, val);
            variables.insert(param.name.clone(), var);
        }

        let mut is_terminated = false;
        
        {
            let mut gen_ctx = GenContext {
                builder: &mut builder,
                module,
                variables: &mut variables,
                var_counter: &mut var_counter,
                loop_stack: &mut loop_stack,
                printf_id,
                malloc_id,
                realloc_id,
                memmove_id,
                gul_concat_id,
                gul_int_to_str_id,
                gul_float_to_str_id,
                gul_print_float_id,
                strcmp_id,
                format_int_id,
                format_str_id,
                format_true_id,
                format_false_id,
                string_literals,
                data_description: &mut self.data_description,
            };

            for stmt in body {
                if Self::generate_body_statement(stmt, &mut gen_ctx)? {
                    is_terminated = true;
                    break;
                }
            }
            
            if !is_terminated {
                 let zero = gen_ctx.builder.ins().iconst(types::I64, 0);
                 gen_ctx.builder.ins().return_(&[zero]);
            }
        }
        
        builder.finalize();
        
        module.define_function(func_id, ctx)
            .map_err(|e| anyhow!("Failed to define function: {}", e))?;
        module.clear_context(ctx);
        
        Ok(())
    }

    fn generate_body_statement(
        stmt: &Statement,
        ctx: &mut GenContext,
    ) -> Result<bool> {
        match stmt {
            Statement::Let { name, ty: _, value } | Statement::Var { name, ty: _, value } => {
                let val = Self::generate_expression(value, ctx)?;
                
                let ty = if let AstType::Float = value.ty { types::F64 } else { types::I64 };
                let var = if let Some(existing_var) = ctx.variables.get(name) {
                    *existing_var
                } else {
                    let var = Variable::new(*ctx.var_counter);
                    *ctx.var_counter += 1;
                    ctx.variables.insert(name.clone(), var);
                    ctx.builder.declare_var(var, ty);
                    var
                };
                
                ctx.builder.def_var(var, val);
                Ok(false)
            }
            Statement::Assignment { name, value } => {
                let val = Self::generate_expression(value, ctx)?;
                 if let Some(var) = ctx.variables.get(name) {
                    ctx.builder.def_var(*var, val);
                } else {
                    return Err(anyhow!("Undefined variable in assignment: {}", name));
                }
                Ok(false)
            }
            Statement::SetIndex { target, index, value } => {
                let header_ptr = Self::generate_expression(target, ctx)?;
                let idx = Self::generate_expression(index, ctx)?;
                let val = Self::generate_expression(value, ctx)?;
                
                // New Copy-Semantics: header[8] is data_ptr
                let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);

                let eight = ctx.builder.ins().iconst(types::I64, 8);
                let offset_mul = ctx.builder.ins().imul(idx, eight);
                let elem_addr = ctx.builder.ins().iadd(data_ptr, offset_mul);
                
                ctx.builder.ins().store(MemFlags::new(), val, elem_addr, 0);
                Ok(false)
            }
            Statement::Return { value } => {
                let ret_val = if let Some(expr) = value {
                    Self::generate_expression(expr, ctx)?
                } else {
                    ctx.builder.ins().iconst(types::I64, 0)
                };
                ctx.builder.ins().return_(&[ret_val]);
                Ok(true)
            }
            Statement::Expression { expr } => {
                Self::generate_expression(expr, ctx)?;
                Ok(false)
            }
            Statement::If { condition, then_branch, else_branch } => {
                let condition_val = Self::generate_expression(condition, ctx)?;
                
                let then_block = ctx.builder.create_block();
                let else_block = ctx.builder.create_block();
                let merge_block = ctx.builder.create_block();
                
                ctx.builder.ins().brif(condition_val, then_block, &[], else_block, &[]);
                
                ctx.builder.switch_to_block(then_block);
                ctx.builder.seal_block(then_block);
                let mut then_term = false;
                for s in then_branch {
                    if Self::generate_body_statement(s, ctx)? {
                        then_term = true; 
                        break;
                    }
                }
                if !then_term { ctx.builder.ins().jump(merge_block, &[]); }
                
                ctx.builder.switch_to_block(else_block);
                ctx.builder.seal_block(else_block);
                let mut else_term = false;
                if let Some(else_stmts) = else_branch {
                    for s in else_stmts {
                        if Self::generate_body_statement(s, ctx)? {
                            else_term = true;
                            break;
                        }
                    }
                }
                if !else_term { ctx.builder.ins().jump(merge_block, &[]); }
                
                ctx.builder.switch_to_block(merge_block);
                ctx.builder.seal_block(merge_block);
                
                Ok(then_term && else_term)
            }
            Statement::While { condition, body } => {
                let header = ctx.builder.create_block();
                let body_block = ctx.builder.create_block();
                let exit = ctx.builder.create_block();
                
                ctx.builder.ins().jump(header, &[]);
                ctx.builder.switch_to_block(header);
                
                let cond_val = Self::generate_expression(condition, ctx)?;
                ctx.builder.ins().brif(cond_val, body_block, &[], exit, &[]);
                
                ctx.builder.switch_to_block(body_block);
                ctx.loop_stack.push(LoopContext { header_block: header, exit_block: exit });
                
                let mut body_term = false;
                for s in body {
                    if Self::generate_body_statement(s, ctx)? {
                        body_term = true;
                        break;
                    }
                }
                ctx.loop_stack.pop();
                
                if !body_term { ctx.builder.ins().jump(header, &[]); }
                
                ctx.builder.switch_to_block(exit);
                ctx.builder.seal_block(header);
                ctx.builder.seal_block(body_block);
                ctx.builder.seal_block(exit);
                
                Ok(false)
            }
            Statement::Loop { body } => {
                let header = ctx.builder.create_block();
                let exit = ctx.builder.create_block();
                
                ctx.builder.ins().jump(header, &[]);
                ctx.builder.switch_to_block(header);
                
                ctx.loop_stack.push(LoopContext { header_block: header, exit_block: exit });
                
                let mut body_term = false;
                for s in body {
                   if Self::generate_body_statement(s, ctx)? {
                       body_term = true;
                       break;
                   }
                }
                ctx.loop_stack.pop();
                
                if !body_term { ctx.builder.ins().jump(header, &[]); }
                
                ctx.builder.switch_to_block(exit);
                ctx.builder.seal_block(header);
                ctx.builder.seal_block(exit);
                Ok(false) 
            }
            Statement::Match { value, arms } => {
                let val = Self::generate_expression(value, ctx)?;
                
                let merge_block = ctx.builder.create_block();
                let mut current_check_block = ctx.builder.create_block();
                ctx.builder.ins().jump(current_check_block, &[]);
                
                for arm in arms {
                    ctx.builder.switch_to_block(current_check_block);
                    ctx.builder.seal_block(current_check_block);
                    
                    let body_block = ctx.builder.create_block();
                    let next_check_block = ctx.builder.create_block();
                    
                    match &arm.pattern {
                        Pattern::Literal(lit_kind) => {
                             match lit_kind {
                                 ExpressionKind::String(s) => {
                                     // Generate string literal for pattern
                                     let pat_expr = Expression{ kind: ExpressionKind::String(s.clone()), ty: AstType::String };
                                     let pat_val = Self::generate_expression(&pat_expr, ctx)?;
                                     
                                     // Call strcmp
                                     let strcmp_ref = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                                     let call = ctx.builder.ins().call(strcmp_ref, &[val, pat_val]);
                                     let res = ctx.builder.inst_results(call)[0]; // i32
                                     // eq if res == 0
                                     let cmp = ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0);
                                     ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
                                 },
                                  ExpressionKind::Integer(n) => {
                                      if matches!(value.ty, AstType::Float) {
                                           let pat_val = ctx.builder.ins().f64const(*n as f64);
                                           let cmp = ctx.builder.ins().fcmp(FloatCC::Equal, val, pat_val);
                                           ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
                                      } else {
                                           let pat_val = ctx.builder.ins().iconst(types::I64, *n);
                                           let cmp = ctx.builder.ins().icmp(IntCC::Equal, val, pat_val);
                                           ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
                                      }
                                  },
                                 ExpressionKind::Boolean(b) => {
                                      let pat_val = ctx.builder.ins().iconst(types::I8, if *b { 1 } else { 0 });
                                      let cmp = ctx.builder.ins().icmp(IntCC::Equal, val, pat_val);
                                      ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
                                 },
                                 // Float
                                 ExpressionKind::Float(f) => {
                                     let pat_val = ctx.builder.ins().f64const(*f);
                                      // Float eq
                                      let cmp = ctx.builder.ins().fcmp(FloatCC::Equal, val, pat_val);
                                      ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
                                 },
                                 _ => {
                                     ctx.builder.ins().jump(next_check_block, &[]);
                                 }
                             }
                        },
                        Pattern::Wildcard => {
                            ctx.builder.ins().jump(body_block, &[]);
                        },
                        Pattern::Identifier(_) => {
                             ctx.builder.ins().jump(body_block, &[]);
                        }
                    }
                    
                    // Generate Body
                    ctx.builder.switch_to_block(body_block);
                    ctx.builder.seal_block(body_block);
                    
                    if let Pattern::Identifier(name) = &arm.pattern {
                         let var = Variable::new(*ctx.var_counter);
                         *ctx.var_counter += 1;
                          ctx.variables.insert(name.clone(), var);
                          
                          let ty = if let AstType::Float = value.ty { types::F64 } else { types::I64 };
                          ctx.builder.declare_var(var, ty);
                         ctx.builder.def_var(var, val);
                    }
                    
                    let mut arm_term = false;
                    for s in &arm.body {
                        if Self::generate_body_statement(s, ctx)? {
                            arm_term = true;
                            break;
                        }
                    }
                    if !arm_term {
                        ctx.builder.ins().jump(merge_block, &[]);
                    }
                    
                    current_check_block = next_check_block;
                }
                
                // Final fallback
                ctx.builder.switch_to_block(current_check_block);
                ctx.builder.seal_block(current_check_block);
                ctx.builder.ins().jump(merge_block, &[]);
                
                ctx.builder.switch_to_block(merge_block);
                ctx.builder.seal_block(merge_block);
                
                Ok(false)
            }
            Statement::Break => {
                if let Some(lctx) = ctx.loop_stack.last() {
                    ctx.builder.ins().jump(lctx.exit_block, &[]);
                    Ok(true)
                } else { Ok(false) }
            }
            Statement::Continue => {
                if let Some(lctx) = ctx.loop_stack.last() {
                    ctx.builder.ins().jump(lctx.header_block, &[]);
                    Ok(true)
                } else { Ok(false) }
            }
            _ => Ok(false),
        }
    }

    fn generate_expression(
        expr: &Expression, 
        ctx: &mut GenContext
    ) -> Result<Value> {
        match &expr.kind {
            ExpressionKind::Integer(n) => Ok(ctx.builder.ins().iconst(types::I64, *n)),
            ExpressionKind::Float(n) => Ok(ctx.builder.ins().f64const(*n)),
            ExpressionKind::Boolean(b) => Ok(ctx.builder.ins().iconst(types::I8, if *b { 1 } else { 0 })),
            
            ExpressionKind::String(val) => {
                let data_id = if let Some(id) = ctx.string_literals.get(val) {
                    *id
                } else {
                    let mut bytes = val.as_bytes().to_vec();
                    bytes.push(0); 
                    ctx.data_description.define(bytes.into_boxed_slice());
                    let id = ctx.module.declare_data(
                        &format!("str_{}", ctx.string_literals.len()), 
                        Linkage::Local, true, false
                    ).map_err(|e| anyhow!("Failed to declare string data: {}", e))?;
                    ctx.module.define_data(id, ctx.data_description)
                         .map_err(|e| anyhow!("Failed to define string data: {}", e))?;
                    ctx.data_description.clear();
                    ctx.string_literals.insert(val.clone(), id);
                    id
                };
                let data_val = ctx.module.declare_data_in_func(data_id, ctx.builder.func);
                Ok(ctx.builder.ins().global_value(types::I64, data_val))
            },
            
            ExpressionKind::Identifier(name) => {
                if let Some(var) = ctx.variables.get(name) {
                    Ok(ctx.builder.use_var(*var))
                } else {
                    Err(anyhow!("Undefined variable: {}", name))
                }
            },
            ExpressionKind::List(items) => {
                let len = items.len();
                let data_size = (len * 8) as i64;
                // malloc(0) is unsafe/UB, alloc at least 8 bytes if empty
                let data_size_val = ctx.builder.ins().iconst(types::I64, if data_size == 0 { 8 } else { data_size });
                
                let malloc_func = ctx.module.declare_func_in_func(ctx.malloc_id, ctx.builder.func);
                
                // 1. Allocate Data
                let call_data = ctx.builder.ins().call(malloc_func, &[data_size_val]);
                let data_ptr = ctx.builder.inst_results(call_data)[0];
                
                // Store items into data
                for (i, item) in items.iter().enumerate() {
                    let val = Self::generate_expression(item, ctx)?;
                    let offset = (i * 8) as i32;
                    ctx.builder.ins().store(MemFlags::new(), val, data_ptr, offset);
                }
                
                // 2. Allocate Header (16 bytes: len + data_ptr)
                let header_size_val = ctx.builder.ins().iconst(types::I64, 16);
                let call_header = ctx.builder.ins().call(malloc_func, &[header_size_val]);
                let header_ptr = ctx.builder.inst_results(call_header)[0];
                
                // Store Len
                let len_val = ctx.builder.ins().iconst(types::I64, len as i64);
                ctx.builder.ins().store(MemFlags::new(), len_val, header_ptr, 0);
                
                // Store Data Ptr
                ctx.builder.ins().store(MemFlags::new(), data_ptr, header_ptr, 8);
                
                Ok(header_ptr)
            },
            ExpressionKind::Set(items) => {
                 // For MVP, Set uses same memory layout as List
                 // We rely on `add` and parser to enforce uniqueness if we were strict
                 // But for now, just allocate similarly
                 
                 let len = items.len();
                 let data_size = (len * 8) as i64;
                 let data_size_val = ctx.builder.ins().iconst(types::I64, if data_size == 0 { 8 } else { data_size });
                 
                 let malloc_func = ctx.module.declare_func_in_func(ctx.malloc_id, ctx.builder.func);
                 
                 let call_data = ctx.builder.ins().call(malloc_func, &[data_size_val]);
                 let data_ptr = ctx.builder.inst_results(call_data)[0];
                 
                 for (i, item) in items.iter().enumerate() {
                     let val = Self::generate_expression(item, ctx)?;
                     let offset = (i * 8) as i32;
                     ctx.builder.ins().store(MemFlags::new(), val, data_ptr, offset);
                 }
                 
                 let header_size_val = ctx.builder.ins().iconst(types::I64, 16);
                 let call_header = ctx.builder.ins().call(malloc_func, &[header_size_val]);
                 let header_ptr = ctx.builder.inst_results(call_header)[0];
                 
                 let len_val = ctx.builder.ins().iconst(types::I64, len as i64);
                 ctx.builder.ins().store(MemFlags::new(), len_val, header_ptr, 0);
                 ctx.builder.ins().store(MemFlags::new(), data_ptr, header_ptr, 8);
                 
                 Ok(header_ptr)

            },
            ExpressionKind::Dict(items) => {
                 // Dict MVP: flat array of [Key, Value, Key, Value...]
                 // Capacity = len * 2 * 8 bytes (16 bytes per pair)
                 // Header same as List: [Len | DataPtr]
                 // Here Len is number of Pairs.
                 
                 let len = items.len();
                 let data_size = (len * 16) as i64; // 2 * 8 per item
                 let data_size_val = ctx.builder.ins().iconst(types::I64, if data_size == 0 { 8 } else { data_size });
                 
                 let malloc_func = ctx.module.declare_func_in_func(ctx.malloc_id, ctx.builder.func);
                 let call_data = ctx.builder.ins().call(malloc_func, &[data_size_val]);
                 let data_ptr = ctx.builder.inst_results(call_data)[0];
                 
                 for (i, (key, value)) in items.iter().enumerate() {
                     let key_val = Self::generate_expression(key, ctx)?;
                     let val_val = Self::generate_expression(value, ctx)?;
                     
                     // Key offset: i * 16
                     // Val offset: i * 16 + 8
                     
                     let i_16 = (i * 16) as i32;
                     ctx.builder.ins().store(MemFlags::new(), key_val, data_ptr, i_16);
                     ctx.builder.ins().store(MemFlags::new(), val_val, data_ptr, i_16 + 8);
                 }
                 
                 let header_size_val = ctx.builder.ins().iconst(types::I64, 16);
                 let call_header = ctx.builder.ins().call(malloc_func, &[header_size_val]);
                 let header_ptr = ctx.builder.inst_results(call_header)[0];
                 
                 let len_val = ctx.builder.ins().iconst(types::I64, len as i64);
                 ctx.builder.ins().store(MemFlags::new(), len_val, header_ptr, 0);
                 ctx.builder.ins().store(MemFlags::new(), data_ptr, header_ptr, 8);
                 
                 Ok(header_ptr)
            },
            ExpressionKind::MemberAccess { object, member } => {
                if member == "len" {
                    let header_ptr = Self::generate_expression(object, ctx)?;
                    Ok(ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0))
                } else {
                     Err(anyhow!("Unknown member property: {}", member))
                }
            },
            ExpressionKind::Index { target, index } => {
                let header_ptr = Self::generate_expression(target, ctx)?;
                let idx_or_key = Self::generate_expression(index, ctx)?;
                
                // If target is Dict, do linear search. Else List index.
                let is_dict = matches!(target.ty, AstType::Dict(_, _));
                
                if is_dict {
                    let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                    let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                    
                    let loop_block = ctx.builder.create_block();
                    let body_block = ctx.builder.create_block();
                    let next_block = ctx.builder.create_block();
                    let found_block = ctx.builder.create_block();
                    let not_found_block = ctx.builder.create_block(); // Return 0 or panic? 0 for now
                    let exit_block = ctx.builder.create_block();
                    
                    let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                     ctx.builder.declare_var(i_var, types::I64);
                     let zero = ctx.builder.ins().iconst(types::I64, 0);
                     ctx.builder.def_var(i_var, zero);
                     
                     let res_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                     ctx.builder.declare_var(res_var, types::I64);
                     let zero_res = ctx.builder.ins().iconst(types::I64, 0);
                     ctx.builder.def_var(res_var, zero_res);

                    ctx.builder.ins().jump(loop_block, &[]);

                    ctx.builder.switch_to_block(loop_block);
                    let i = ctx.builder.use_var(i_var);
                    let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i, len);
                    ctx.builder.ins().brif(cmp_len, body_block, &[], not_found_block, &[]);

                    ctx.builder.switch_to_block(body_block);
                    // Key at i*16
                    let sixteen = ctx.builder.ins().iconst(types::I64, 16);
                    let offset = ctx.builder.ins().imul(i, sixteen);
                    let key_addr = ctx.builder.ins().iadd(data_ptr, offset);
                    let key = ctx.builder.ins().load(types::I64, MemFlags::new(), key_addr, 0);
                    
                     let found_cmp = if matches!(index.ty, AstType::String) {
                          let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                          let call = ctx.builder.ins().call(strcmp_func, &[key, idx_or_key]);
                          let res = ctx.builder.inst_results(call)[0];
                          ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                      } else {
                          ctx.builder.ins().icmp(IntCC::Equal, key, idx_or_key)
                      };
                    ctx.builder.ins().brif(found_cmp, found_block, &[], next_block, &[]);

                    ctx.builder.switch_to_block(found_block);
                    // Value at i*16 + 8
                    let eight = ctx.builder.ins().iconst(types::I64, 8);
                    let val_addr = ctx.builder.ins().iadd(key_addr, eight);
                    let val = ctx.builder.ins().load(types::I64, MemFlags::new(), val_addr, 0);
                    ctx.builder.def_var(res_var, val);
                    ctx.builder.ins().jump(exit_block, &[]);

                    ctx.builder.switch_to_block(next_block);
                    let next_i = ctx.builder.ins().iadd_imm(i, 1);
                    ctx.builder.def_var(i_var, next_i);
                    ctx.builder.ins().jump(loop_block, &[]);
                    
                    ctx.builder.switch_to_block(not_found_block);
                    // Maybe print error "Key not found"?
                    // For now return 0
                    ctx.builder.ins().jump(exit_block, &[]);
                    
                    ctx.builder.switch_to_block(exit_block);
                    ctx.builder.seal_block(loop_block);
                    ctx.builder.seal_block(body_block);
                    ctx.builder.seal_block(found_block);
                    ctx.builder.seal_block(next_block);
                    ctx.builder.seal_block(not_found_block);
                    ctx.builder.seal_block(exit_block);
                    
                    Ok(ctx.builder.use_var(res_var))
                    
                } else {
                    let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                    
                    let eight = ctx.builder.ins().iconst(types::I64, 8);
                    let offset_mul = ctx.builder.ins().imul(idx_or_key, eight);
                    let elem_addr = ctx.builder.ins().iadd(data_ptr, offset_mul);
                    
                    Ok(ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0))
                }
            },
        ExpressionKind::Call { callee, args } => {
            if let ExpressionKind::MemberAccess { object, member } = &callee.kind {
                 if member == "push" || member == "add" {
                     // Check if it's a Dictionary
                     if let AstType::Dict(_, _) = object.ty {
                         // Dict add(key, val)
                         if args.len() != 2 { return Err(anyhow!("Dict.{} expects 2 args: key, value", member)); }
                         
                         let key_val = Self::generate_expression(&args[0], ctx)?;
                         let val_val = Self::generate_expression(&args[1], ctx)?;
                         let header_ptr = Self::generate_expression(object, ctx)?;
                         
                         let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                         let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                         
                         let one = ctx.builder.ins().iconst(types::I64, 1);
                         let new_len = ctx.builder.ins().iadd(len, one); // +1 pair
                         
                         let sixteen = ctx.builder.ins().iconst(types::I64, 16);
                         let new_size = ctx.builder.ins().imul(new_len, sixteen);
                         
                         let realloc_func = ctx.module.declare_func_in_func(ctx.realloc_id, ctx.builder.func);
                         let call = ctx.builder.ins().call(realloc_func, &[data_ptr, new_size]);
                         let new_data_ptr = ctx.builder.inst_results(call)[0];
                         
                         ctx.builder.ins().store(MemFlags::new(), new_data_ptr, header_ptr, 8);
                         ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                         
                         // Store Pair at offset len*16
                         let offset = ctx.builder.ins().imul(len, sixteen);
                         let key_addr = ctx.builder.ins().iadd(new_data_ptr, offset);
                         let offset_8 = ctx.builder.ins().iconst(types::I64, 8);
                         let val_addr = ctx.builder.ins().iadd(key_addr, offset_8);
                         
                         ctx.builder.ins().store(MemFlags::new(), key_val, key_addr, 0);
                         ctx.builder.ins().store(MemFlags::new(), val_val, val_addr, 0);
                         
                         return Ok(ctx.builder.ins().iconst(types::I64, 0));
                     }
                     
                     // List add(val)
                     if args.len() != 1 { return Err(anyhow!("{} expects 1 arg", member)); }
                     let item_val = Self::generate_expression(&args[0], ctx)?;
                     let header_ptr = Self::generate_expression(object, ctx)?;
                     
                     let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                     let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                     
                     let one = ctx.builder.ins().iconst(types::I64, 1);
                     let new_len = ctx.builder.ins().iadd(len, one);
                     
                     let eight = ctx.builder.ins().iconst(types::I64, 8);
                     let new_size = ctx.builder.ins().imul(new_len, eight);
                     
                     let realloc_func = ctx.module.declare_func_in_func(ctx.realloc_id, ctx.builder.func);
                     let call = ctx.builder.ins().call(realloc_func, &[data_ptr, new_size]);
                     let new_data_ptr = ctx.builder.inst_results(call)[0];
                     
                     ctx.builder.ins().store(MemFlags::new(), new_data_ptr, header_ptr, 8);
                     ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                     
                     // Store item at offset len*8
                     let offset = ctx.builder.ins().imul(len, eight);
                     let elem_addr = ctx.builder.ins().iadd(new_data_ptr, offset);
                     ctx.builder.ins().store(MemFlags::new(), item_val, elem_addr, 0);
                     
                     return Ok(ctx.builder.ins().iconst(types::I64, 0));
                     } else if member == "insertbefore" {
                      // insertbefore(index, value) -> 2 args
                      // insertbefore(value) -> 1 arg (prepend) implies index 0
                      
                      let (idx_val, item_val) = if args.len() == 2 {
                          (Self::generate_expression(&args[0], ctx)?, Self::generate_expression(&args[1], ctx)?)
                      } else if args.len() == 1 {
                          (ctx.builder.ins().iconst(types::I64, 0), Self::generate_expression(&args[0], ctx)?)
                      } else {
                          return Err(anyhow!("insertbefore expects 1 or 2 args"));
                      };

                      let header_ptr = Self::generate_expression(object, ctx)?;


                      let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                      let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);

                      // Resize +1
                      let one = ctx.builder.ins().iconst(types::I64, 1);
                      let new_len = ctx.builder.ins().iadd(len, one);
                      let eight = ctx.builder.ins().iconst(types::I64, 8);
                      let new_size = ctx.builder.ins().imul(new_len, eight);

                      let realloc_func = ctx.module.declare_func_in_func(ctx.realloc_id, ctx.builder.func);
                      let call_realloc = ctx.builder.ins().call(realloc_func, &[data_ptr, new_size]);
                      let new_data_ptr = ctx.builder.inst_results(call_realloc)[0];

                      ctx.builder.ins().store(MemFlags::new(), new_data_ptr, header_ptr, 8);
                      ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);

                      // memmove:
                      // dest = data + (idx + 1) * 8
                      // src = data + idx * 8
                      // count = (len - idx) * 8
                      
                      let idx_offset = ctx.builder.ins().imul(idx_val, eight);
                      let src_ptr = ctx.builder.ins().iadd(new_data_ptr, idx_offset);
                      let dest_ptr = ctx.builder.ins().iadd(src_ptr, eight);

                      let count_elems = ctx.builder.ins().isub(len, idx_val);
                      let count_bytes = ctx.builder.ins().imul(count_elems, eight);
                      
                      let memmove_func = ctx.module.declare_func_in_func(ctx.memmove_id, ctx.builder.func);
                      // check if count > 0 before memmove? memmove handles 0 fine usually.
                      ctx.builder.ins().call(memmove_func, &[dest_ptr, src_ptr, count_bytes]);
                      
                      // store val at src_ptr (which is data + idx*8)
                      ctx.builder.ins().store(MemFlags::new(), item_val, src_ptr, 0);
                      
                      return Ok(ctx.builder.ins().iconst(types::I64, 0));
                 } else if member == "insertafter" {
                      // insertafter(index, value) -> insert at index + 1
                      // insertafter(value) -> push(value)
                      
                      if args.len() == 1 {
                          // redirect to push logic manually or just impl generic append
                          // reusing push logic is complex due to context borrow, so just reimplement simple append
                          let item_val = Self::generate_expression(&args[0], ctx)?;
                          let header_ptr = Self::generate_expression(object, ctx)?;
                          
                          let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                          let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                          // Resize + 1
                          let one = ctx.builder.ins().iconst(types::I64, 1);
                          let new_len = ctx.builder.ins().iadd(len, one);
                          let eight = ctx.builder.ins().iconst(types::I64, 8);
                          let new_size = ctx.builder.ins().imul(new_len, eight);
                          let realloc_func = ctx.module.declare_func_in_func(ctx.realloc_id, ctx.builder.func);
                          let call = ctx.builder.ins().call(realloc_func, &[data_ptr, new_size]);
                          let new_data_ptr = ctx.builder.inst_results(call)[0];
                          
                          ctx.builder.ins().store(MemFlags::new(), new_data_ptr, header_ptr, 8);
                          ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                          
                          let offset = ctx.builder.ins().imul(len, eight);
                          let elem_addr = ctx.builder.ins().iadd(new_data_ptr, offset);
                          ctx.builder.ins().store(MemFlags::new(), item_val, elem_addr, 0);
                          
                          return Ok(ctx.builder.ins().iconst(types::I64, 0));
                      } else if args.len() == 2 {
                          // insertafter(idx, val) => insertbefore(idx+1, val)
                           let idx_val = Self::generate_expression(&args[0], ctx)?;
                           let item_val = Self::generate_expression(&args[1], ctx)?;
                           let header_ptr = Self::generate_expression(object, ctx)?;
                           
                           // Reuse insert logic
                           let one = ctx.builder.ins().iconst(types::I64, 1);
                           let target_idx = ctx.builder.ins().iadd(idx_val, one);
                           
                           // ... (Code duplication for insert logic due to borrow checker constraints on self calling)
                           // Implementation of insert at target_idx
                           let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                           let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                           
                           let new_len = ctx.builder.ins().iadd(len, one);
                           let eight = ctx.builder.ins().iconst(types::I64, 8);
                           let new_size = ctx.builder.ins().imul(new_len, eight);
                           let realloc_func = ctx.module.declare_func_in_func(ctx.realloc_id, ctx.builder.func);
                           let call = ctx.builder.ins().call(realloc_func, &[data_ptr, new_size]);
                           let new_data_ptr = ctx.builder.inst_results(call)[0];
                           
                           ctx.builder.ins().store(MemFlags::new(), new_data_ptr, header_ptr, 8);
                           ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                           
                           // memmove
                           let idx_offset = ctx.builder.ins().imul(target_idx, eight);
                           let src_ptr = ctx.builder.ins().iadd(new_data_ptr, idx_offset);
                           let dest_ptr = ctx.builder.ins().iadd(src_ptr, eight);
                           
                           let count_elems = ctx.builder.ins().isub(len, target_idx);
                           let count_bytes = ctx.builder.ins().imul(count_elems, eight);
                           
                           let memmove_func = ctx.module.declare_func_in_func(ctx.memmove_id, ctx.builder.func);
                           ctx.builder.ins().call(memmove_func, &[dest_ptr, src_ptr, count_bytes]);
                           
                           ctx.builder.ins().store(MemFlags::new(), item_val, src_ptr, 0);
                           
                           return Ok(ctx.builder.ins().iconst(types::I64, 0));
                      } else {
                          return Err(anyhow!("insertafter expects 1 or 2 args"));
                      }
                 } else if member == "pop" {
                      // pop() -> remove last, return it
                      // pop(index) -> remove at index, return it
                      
                      let header_ptr = Self::generate_expression(object, ctx)?;
                      let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                      let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                      let eight = ctx.builder.ins().iconst(types::I64, 8);
                      
                      let index = if args.len() == 0 {
                           ctx.builder.ins().iadd_imm(len, -1) // last index
                      } else if args.len() == 1 {
                           Self::generate_expression(&args[0], ctx)?
                      } else {
                           return Err(anyhow!("pop takes 0 or 1 args"));
                      };
                      
                      // Load return value
                      let offset = ctx.builder.ins().imul(index, eight);
                      let elem_addr = ctx.builder.ins().iadd(data_ptr, offset);
                      let ret_val = ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0);
                      
                      // Generic shift logic
                      // dest = data + index*8 (elem_addr)
                      // src = data + (index+1)*8
                      // count = (len - (index+1)) * 8
                      
                      let index_plus_1 = ctx.builder.ins().iadd_imm(index, 1);
                      let src_offset = ctx.builder.ins().imul(index_plus_1, eight);
                      let src_ptr = ctx.builder.ins().iadd(data_ptr, src_offset);
                      let dest_ptr = elem_addr;
                      
                      let count_elems = ctx.builder.ins().isub(len, index_plus_1);
                      let count_bytes = ctx.builder.ins().imul(count_elems, eight);
                      
                      let memmove_func = ctx.module.declare_func_in_func(ctx.memmove_id, ctx.builder.func);
                      ctx.builder.ins().call(memmove_func, &[dest_ptr, src_ptr, count_bytes]);
                      
                       // Dec len
                      let new_len = ctx.builder.ins().iadd_imm(len, -1);
                      ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                      
                      return Ok(ret_val);

                 } else if member == "remove" {
                      // remove(value) -> bool (List/Set)
                      // remove(key) -> bool (Dict)
                      
                      if args.len() != 1 { return Err(anyhow!("remove expects 1 arg")); }
                      let arg_val = Self::generate_expression(&args[0], ctx)?;
                      let header_ptr = Self::generate_expression(object, ctx)?;
                      
                      // Check Dict
                      if let AstType::Dict(_, _) = object.ty {
                          // Dict Remove
                          let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                          let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                          
                          let loop_block = ctx.builder.create_block();
                          let body_block = ctx.builder.create_block();
                          let next_block = ctx.builder.create_block();
                          let found_block = ctx.builder.create_block();
                          let not_found_block = ctx.builder.create_block();
                          let exit_block = ctx.builder.create_block();
                          
                          let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                          ctx.builder.declare_var(i_var, types::I64);
                          let zero = ctx.builder.ins().iconst(types::I64, 0);
                          ctx.builder.def_var(i_var, zero);
                          
                          let res_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                          ctx.builder.declare_var(res_var, types::I64);
                          ctx.builder.def_var(res_var, zero); // default 0/false
                          
                          ctx.builder.ins().jump(loop_block, &[]);
                          
                          ctx.builder.switch_to_block(loop_block);
                          let i = ctx.builder.use_var(i_var);
                          let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i, len);
                          ctx.builder.ins().brif(cmp_len, body_block, &[], not_found_block, &[]);
                          
                          ctx.builder.switch_to_block(body_block);
                          let sixteen = ctx.builder.ins().iconst(types::I64, 16);
                          let offset = ctx.builder.ins().imul(i, sixteen);
                          let key_addr = ctx.builder.ins().iadd(data_ptr, offset);
                          let key_val = ctx.builder.ins().load(types::I64, MemFlags::new(), key_addr, 0);
                          
                          let found_cmp = if matches!(args[0].ty, AstType::String) {
                              let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                              let call = ctx.builder.ins().call(strcmp_func, &[key_val, arg_val]);
                              let res = ctx.builder.inst_results(call)[0];
                              ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                          } else {
                              ctx.builder.ins().icmp(IntCC::Equal, key_val, arg_val)
                          };
                          ctx.builder.ins().brif(found_cmp, found_block, &[], next_block, &[]);
                          
                          ctx.builder.switch_to_block(next_block);
                          let next_i = ctx.builder.ins().iadd_imm(i, 1);
                          ctx.builder.def_var(i_var, next_i);
                          ctx.builder.ins().jump(loop_block, &[]);
                          
                          ctx.builder.switch_to_block(found_block);
                          // Move logic (16 bytes per item)
                          // dest = data + i*16
                          // src = data + (i+1)*16
                          
                          let i_plus_1 = ctx.builder.ins().iadd_imm(i, 1);
                          let src_offset = ctx.builder.ins().imul(i_plus_1, sixteen);
                          let src_ptr = ctx.builder.ins().iadd(data_ptr, src_offset);
                          let dest_ptr = key_addr; 
                          
                          let count_pairs = ctx.builder.ins().isub(len, i_plus_1);
                          let count_bytes = ctx.builder.ins().imul(count_pairs, sixteen);
                          
                          let memmove_func = ctx.module.declare_func_in_func(ctx.memmove_id, ctx.builder.func);
                          ctx.builder.ins().call(memmove_func, &[dest_ptr, src_ptr, count_bytes]);
                          
                          let new_len = ctx.builder.ins().iadd_imm(len, -1);
                          ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                          
                          let one = ctx.builder.ins().iconst(types::I64, 1);
                          ctx.builder.def_var(res_var, one);
                          ctx.builder.ins().jump(exit_block, &[]);
                          
                          ctx.builder.switch_to_block(not_found_block);
                          ctx.builder.ins().jump(exit_block, &[]);
                          
                          ctx.builder.switch_to_block(exit_block);
                          ctx.builder.seal_block(loop_block);
                          ctx.builder.seal_block(body_block);
                          ctx.builder.seal_block(found_block);
                          ctx.builder.seal_block(next_block);
                          ctx.builder.seal_block(not_found_block);
                          ctx.builder.seal_block(exit_block);
                          
                          return Ok(ctx.builder.use_var(res_var));
                      }
                      
                      // List/Set Remove (already implemented below) ->
                      let item_val = arg_val;
                      // ... (Keep existing List logic)
                      
                      let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                      let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);

                      
                      let loop_block = ctx.builder.create_block();
                      let body_block = ctx.builder.create_block();
                      let next_block = ctx.builder.create_block();
                      let found_block = ctx.builder.create_block();
                      let not_found_block = ctx.builder.create_block();
                      let exit_block = ctx.builder.create_block();
                      
                      let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                      ctx.builder.declare_var(i_var, types::I64);
                      let zero = ctx.builder.ins().iconst(types::I64, 0);
                      ctx.builder.def_var(i_var, zero);
                      
                      let result_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                      ctx.builder.declare_var(result_var, types::I64); // 0 or 1
                      
                      ctx.builder.ins().jump(loop_block, &[]);
                      
                      // LOOP
                      ctx.builder.switch_to_block(loop_block);
                      let i = ctx.builder.use_var(i_var);
                      let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i, len);
                      ctx.builder.ins().brif(cmp_len, body_block, &[], not_found_block, &[]);
                      
                      // BODY
                      ctx.builder.switch_to_block(body_block);
                      let eight = ctx.builder.ins().iconst(types::I64, 8);
                      let offset = ctx.builder.ins().imul(i, eight);
                      let elem_addr = ctx.builder.ins().iadd(data_ptr, offset);
                      let elem = ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0);
                      
                      // Compare
                      let found_cmp = if matches!(args[0].ty, AstType::String) { // Roughly checking arg type
                          let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                          let call = ctx.builder.ins().call(strcmp_func, &[elem, item_val]);
                          let res = ctx.builder.inst_results(call)[0];
                          ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                      } else {
                          ctx.builder.ins().icmp(IntCC::Equal, elem, item_val)
                      };
                      ctx.builder.ins().brif(found_cmp, found_block, &[], next_block, &[]);
                      
                      // NEXT
                      ctx.builder.switch_to_block(next_block);
                      let next_i = ctx.builder.ins().iadd_imm(i, 1);
                      ctx.builder.def_var(i_var, next_i);
                      ctx.builder.ins().jump(loop_block, &[]);
                      
                      // FOUND - Remove element at i
                      ctx.builder.switch_to_block(found_block);
                      
                      // dest = data + i*8
                      // src = data + (i+1)*8
                      // count = (len - (i+1)) * 8
                      
                      let i_plus_1 = ctx.builder.ins().iadd_imm(i, 1);
                      let src_offset = ctx.builder.ins().imul(i_plus_1, eight);
                      let src_ptr = ctx.builder.ins().iadd(data_ptr, src_offset);
                      let dest_ptr = elem_addr; // already calc'd
                      
                      let count_elems = ctx.builder.ins().isub(len, i_plus_1);
                      let count_bytes = ctx.builder.ins().imul(count_elems, eight);
                      
                      let memmove_func = ctx.module.declare_func_in_func(ctx.memmove_id, ctx.builder.func);
                      ctx.builder.ins().call(memmove_func, &[dest_ptr, src_ptr, count_bytes]);
                      
                      let new_len = ctx.builder.ins().iadd_imm(len, -1);
                      ctx.builder.ins().store(MemFlags::new(), new_len, header_ptr, 0);
                      
                      let one = ctx.builder.ins().iconst(types::I64, 1);
                      ctx.builder.def_var(result_var, one);
                      ctx.builder.ins().jump(exit_block, &[]);
                      
                      // NOT FOUND
                      ctx.builder.switch_to_block(not_found_block);
                      let zero_res = ctx.builder.ins().iconst(types::I64, 0);
                      ctx.builder.def_var(result_var, zero_res);
                      ctx.builder.ins().jump(exit_block, &[]);
                      
                      // EXIT
                      ctx.builder.switch_to_block(exit_block);
                      ctx.builder.seal_block(loop_block);
                      ctx.builder.seal_block(body_block);
                      ctx.builder.seal_block(found_block);
                      ctx.builder.seal_block(not_found_block);
                      ctx.builder.seal_block(next_block);
                      ctx.builder.seal_block(exit_block);
                      
                      return Ok(ctx.builder.use_var(result_var));
                 } else if member == "clear" {
                      // clear() -> void (or 0)
                      let header_ptr = Self::generate_expression(object, ctx)?;
                      let zero = ctx.builder.ins().iconst(types::I64, 0);
                      ctx.builder.ins().store(MemFlags::new(), zero, header_ptr, 0); // Set len = 0
                      return Ok(zero);
                 } else if member == "len" {
                        // len() Call -> returns length
                        let header_ptr = Self::generate_expression(object, ctx)?;
                        let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                        return Ok(len);
                 } else if member == "contains" || member == "contain" {
                        // contains(val) -> bool (List/Set: search val, Dict: search key)
                        if args.len() != 1 && args.len() != 2 { return Err(anyhow!("contains expects 1 arg (val) or 2 (legacy)")); }
                        // Ignore 2nd arg if present
                        
                        let arg_val = Self::generate_expression(&args[0], ctx)?;
                        let header_ptr = Self::generate_expression(object, ctx)?;
                        
                        if let AstType::Dict(_, _) = object.ty {
                             // Dict Search Key
                            let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                            let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                            
                            let loop_block = ctx.builder.create_block();
                            let body_block = ctx.builder.create_block();
                            let next_block = ctx.builder.create_block();
                            let found_block = ctx.builder.create_block();
                            let exit_block = ctx.builder.create_block();
                            
                            let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                            ctx.builder.declare_var(i_var, types::I64);
                            let zero = ctx.builder.ins().iconst(types::I64, 0);
                            ctx.builder.def_var(i_var, zero);
                            
                            let res_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                            ctx.builder.declare_var(res_var, types::I64);
                            ctx.builder.def_var(res_var, zero);
                            
                            ctx.builder.ins().jump(loop_block, &[]);
                            ctx.builder.switch_to_block(loop_block);
                            let i = ctx.builder.use_var(i_var);
                            let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i, len);
                            ctx.builder.ins().brif(cmp_len, body_block, &[], exit_block, &[]);
                            
                            ctx.builder.switch_to_block(body_block);
                            let sixteen = ctx.builder.ins().iconst(types::I64, 16);
                            let offset = ctx.builder.ins().imul(i, sixteen);
                            let key_addr = ctx.builder.ins().iadd(data_ptr, offset);
                            let key_val = ctx.builder.ins().load(types::I64, MemFlags::new(), key_addr, 0);
                            
                            let found_cmp = if matches!(args[0].ty, AstType::String) {
                                  let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                                  let call = ctx.builder.ins().call(strcmp_func, &[key_val, arg_val]);
                                  let res = ctx.builder.inst_results(call)[0];
                                  ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                              } else {
                                  ctx.builder.ins().icmp(IntCC::Equal, key_val, arg_val)
                              };
                            ctx.builder.ins().brif(found_cmp, found_block, &[], next_block, &[]);
                            
                            ctx.builder.switch_to_block(found_block);
                            let one = ctx.builder.ins().iconst(types::I64, 1);
                            ctx.builder.def_var(res_var, one);
                            ctx.builder.ins().jump(exit_block, &[]);
                            
                            ctx.builder.switch_to_block(next_block);
                            let next_i = ctx.builder.ins().iadd_imm(i, 1);
                            ctx.builder.def_var(i_var, next_i);
                            ctx.builder.ins().jump(loop_block, &[]);
                            
                            ctx.builder.switch_to_block(exit_block);
                            ctx.builder.seal_block(loop_block);
                            ctx.builder.seal_block(body_block);
                            ctx.builder.seal_block(found_block);
                            ctx.builder.seal_block(next_block);
                            ctx.builder.seal_block(exit_block);
                            
                            return Ok(ctx.builder.use_var(res_var));
                        } else {
                            // List/Set Search Value
                            let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                            let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                            
                            let loop_block = ctx.builder.create_block();
                            let body_block = ctx.builder.create_block();
                            let next_block = ctx.builder.create_block();
                            let found_block = ctx.builder.create_block();
                            let exit_block = ctx.builder.create_block();
                            
                            let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                            ctx.builder.declare_var(i_var, types::I64);
                            let zero = ctx.builder.ins().iconst(types::I64, 0);
                            ctx.builder.def_var(i_var, zero);
                            
                            let res_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                            ctx.builder.declare_var(res_var, types::I64);
                            ctx.builder.def_var(res_var, zero);
                            
                            ctx.builder.ins().jump(loop_block, &[]);
                            ctx.builder.switch_to_block(loop_block);
                            let i = ctx.builder.use_var(i_var);
                            let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i, len);
                            ctx.builder.ins().brif(cmp_len, body_block, &[], exit_block, &[]);
                            
                            ctx.builder.switch_to_block(body_block);
                            // List/Set uses 8 bytes per item
                            let eight = ctx.builder.ins().iconst(types::I64, 8);
                            let offset = ctx.builder.ins().imul(i, eight);
                            let elem_addr = ctx.builder.ins().iadd(data_ptr, offset);
                            let elem_val = ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0);
                            
                            let found_cmp = if matches!(args[0].ty, AstType::String) {
                                  let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                                  let call = ctx.builder.ins().call(strcmp_func, &[elem_val, arg_val]);
                                  let res = ctx.builder.inst_results(call)[0];
                                  ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                              } else {
                                  ctx.builder.ins().icmp(IntCC::Equal, elem_val, arg_val)
                              };
                            ctx.builder.ins().brif(found_cmp, found_block, &[], next_block, &[]);
                            
                            ctx.builder.switch_to_block(found_block);
                            let one = ctx.builder.ins().iconst(types::I64, 1);
                            ctx.builder.def_var(res_var, one);
                            ctx.builder.ins().jump(exit_block, &[]);
                            
                             ctx.builder.switch_to_block(next_block);
                            let next_i = ctx.builder.ins().iadd_imm(i, 1);
                            ctx.builder.def_var(i_var, next_i);
                            ctx.builder.ins().jump(loop_block, &[]);
                            
                            ctx.builder.switch_to_block(exit_block);
                            ctx.builder.seal_block(loop_block);
                            ctx.builder.seal_block(body_block);
                            ctx.builder.seal_block(found_block);
                            ctx.builder.seal_block(next_block);
                            ctx.builder.seal_block(exit_block);
                            
                            return Ok(ctx.builder.use_var(res_var));
                        }
                 }
            }

            if let ExpressionKind::Identifier(name) = &callee.kind {
                    if name == "print" {
                        if args.len() != 1 {
                            return Err(anyhow!("print() expects 1 argument"));
                        }
                        let arg_expr = &args[0];
                        let arg_val = Self::generate_expression(arg_expr, ctx)?;
                        
                        if matches!(arg_expr.ty, AstType::Float) {
                             let func_id = ctx.module.declare_func_in_func(ctx.gul_print_float_id, ctx.builder.func);
                             ctx.builder.ins().call(func_id, &[arg_val]);
                             return Ok(ctx.builder.ins().iconst(types::I64, 0));
                        }

                        let fmt_id = match arg_expr.ty {
                            AstType::String => ctx.format_str_id,
                            AstType::Integer => ctx.format_int_id, 
                            AstType::Boolean => ctx.format_str_id, // Print boolean as string
                            _ => ctx.format_int_id, // Default to int (pointers, etc)
                        };
                        
                        // Special handling for boolean printing
                        let final_arg_val = if matches!(arg_expr.ty, AstType::Boolean) {
                            let t_id = ctx.module.declare_data_in_func(ctx.format_true_id, ctx.builder.func);
                            let f_id = ctx.module.declare_data_in_func(ctx.format_false_id, ctx.builder.func);
                            let t_ptr = ctx.builder.ins().global_value(types::I64, t_id);
                            let f_ptr = ctx.builder.ins().global_value(types::I64, f_id);
                            ctx.builder.ins().select(arg_val, t_ptr, f_ptr)
                        } else {
                            arg_val
                        };
                        
                        let fmt_val = ctx.module.declare_data_in_func(fmt_id, ctx.builder.func);
                        let fmt_ptr = ctx.builder.ins().global_value(types::I64, fmt_val);
                        
                        let printf_ref = ctx.module.declare_func_in_func(ctx.printf_id, ctx.builder.func);
                        ctx.builder.ins().call(printf_ref, &[fmt_ptr, final_arg_val]);
                        
                        return Ok(ctx.builder.ins().iconst(types::I64, 0));
                    } else if name == "@flt" || name == "flt" {
                         if args.len() != 1 { return Err(anyhow!("@flt expects 1 argument")); }
                         let arg_expr = &args[0];
                         let arg_val = Self::generate_expression(arg_expr, ctx)?;
                         if matches!(arg_expr.ty, AstType::Integer) {
                             return Ok(ctx.builder.ins().fcvt_from_sint(types::F64, arg_val));
                         } else {
                             return Ok(arg_val); // Assume already float or compatible
                         }
                    } else if name == "@int" || name == "int" {
                         if args.len() != 1 { return Err(anyhow!("@int expects 1 argument")); }
                         let arg_expr = &args[0];
                         let arg_val = Self::generate_expression(arg_expr, ctx)?;
                         if matches!(arg_expr.ty, AstType::Float) {
                             return Ok(ctx.builder.ins().fcvt_to_sint_sat(types::I64, arg_val));
                         } else {
                             return Ok(arg_val); // Assume already int
                         }
                    } else {
                        // General Function Call
                        let mut sig = ctx.module.make_signature();
                        let mut arg_vals = Vec::new();
                        
                        for arg in args {
                             let val = Self::generate_expression(arg, ctx)?;
                             arg_vals.push(val);
                             
                             let ty = if let AstType::Float = arg.ty { types::F64 } else { types::I64 };
                             sig.params.push(AbiParam::new(ty));
                        }
                        sig.returns.push(AbiParam::new(types::I64)); // Assume I64 return
                        
                        let func_id = ctx.module.declare_function(name, Linkage::Export, &sig)
                            .map_err(|e| anyhow!("Failed to declare callee '{}': {}", name, e))?;
                        
                        let local_func = ctx.module.declare_func_in_func(func_id, ctx.builder.func);
                        let call = ctx.builder.ins().call(local_func, &arg_vals);
                        
                        // Functions in GUL currently return I64 (or void=0).
                        let res = ctx.builder.inst_results(call);
                         if res.is_empty() {
                            return Ok(ctx.builder.ins().iconst(types::I64, 0));
                        } else {
                            return Ok(res[0]);
                        }
                    }
                }
                Err(anyhow!("Function calls not fully implemented except 'print'."))
            },
            ExpressionKind::BinaryOp { left, op, right } => {
                let lhs = Self::generate_expression(left, ctx)?;
                let rhs = Self::generate_expression(right, ctx)?;
                
                // Implicit Casts for Mixed Arithmetic
                let lhs = if matches!(left.ty, AstType::Integer) && matches!(right.ty, AstType::Float) {
                    ctx.builder.ins().fcvt_from_sint(types::F64, lhs)
                } else { lhs };
                let rhs = if matches!(right.ty, AstType::Integer) && matches!(left.ty, AstType::Float) {
                    ctx.builder.ins().fcvt_from_sint(types::F64, rhs)
                } else { rhs };
                
                match op {
                    BinaryOperator::Add => {
                        if matches!(left.ty, AstType::String) || matches!(right.ty, AstType::String) {
                            let l_val = if matches!(left.ty, AstType::String) {
                                lhs
                            } else if matches!(left.ty, AstType::Float) {
                                let f2s = ctx.module.declare_func_in_func(ctx.gul_float_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(f2s, &[lhs]);
                                ctx.builder.inst_results(call)[0]
                            } else {
                                let i2s = ctx.module.declare_func_in_func(ctx.gul_int_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(i2s, &[lhs]);
                                ctx.builder.inst_results(call)[0]
                            };
                            
                            let r_val = if matches!(right.ty, AstType::String) {
                                rhs
                            } else if matches!(right.ty, AstType::Float) {
                                let f2s = ctx.module.declare_func_in_func(ctx.gul_float_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(f2s, &[rhs]);
                                ctx.builder.inst_results(call)[0]
                            } else {
                                let i2s = ctx.module.declare_func_in_func(ctx.gul_int_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(i2s, &[rhs]);
                                ctx.builder.inst_results(call)[0]
                            };
                            
                            let concat = ctx.module.declare_func_in_func(ctx.gul_concat_id, ctx.builder.func);
                            let call = ctx.builder.ins().call(concat, &[l_val, r_val]);
                            Ok(ctx.builder.inst_results(call)[0])
                        } else if matches!(left.ty, AstType::Float) || matches!(right.ty, AstType::Float) {
                            Ok(ctx.builder.ins().fadd(lhs, rhs))
                        } else {
                            Ok(ctx.builder.ins().iadd(lhs, rhs))
                        }
                    },
                    BinaryOperator::Subtract => {
                        if matches!(left.ty, AstType::Float) || matches!(right.ty, AstType::Float) {
                            Ok(ctx.builder.ins().fsub(lhs, rhs))
                        } else {
                            Ok(ctx.builder.ins().isub(lhs, rhs))
                        }
                    },
                    BinaryOperator::Multiply => {
                        if matches!(left.ty, AstType::Float) || matches!(right.ty, AstType::Float) {
                             Ok(ctx.builder.ins().fmul(lhs, rhs))
                        } else {
                             Ok(ctx.builder.ins().imul(lhs, rhs))
                        }
                    },
                    BinaryOperator::Divide => {
                        if matches!(left.ty, AstType::Float) || matches!(right.ty, AstType::Float) {
                             Ok(ctx.builder.ins().fdiv(lhs, rhs))
                        } else {
                             Ok(ctx.builder.ins().sdiv(lhs, rhs))
                        }
                    },
                    BinaryOperator::Equal | BinaryOperator::NotEqual | 
                    BinaryOperator::Less | BinaryOperator::Greater |
                    BinaryOperator::LessEq | BinaryOperator::GreaterEq => {
                         if matches!(left.ty, AstType::Float) || matches!(right.ty, AstType::Float) {
                             let cc = match op {
                                BinaryOperator::Equal => FloatCC::Equal,
                                BinaryOperator::NotEqual => FloatCC::NotEqual,
                                BinaryOperator::Less => FloatCC::LessThan,
                                BinaryOperator::Greater => FloatCC::GreaterThan,
                                BinaryOperator::LessEq => FloatCC::LessThanOrEqual,
                                BinaryOperator::GreaterEq => FloatCC::GreaterThanOrEqual,
                                _ => FloatCC::Equal,
                            };
                            let val = ctx.builder.ins().fcmp(cc, lhs, rhs);
                            let true_val = ctx.builder.ins().iconst(types::I64, 1);
                            let false_val = ctx.builder.ins().iconst(types::I64, 0);
                            Ok(ctx.builder.ins().select(val, true_val, false_val))
                         } else {
                             let cc = match op {
                                BinaryOperator::Equal => IntCC::Equal,
                                BinaryOperator::NotEqual => IntCC::NotEqual,
                                BinaryOperator::Less => IntCC::SignedLessThan,
                                BinaryOperator::Greater => IntCC::SignedGreaterThan,
                                BinaryOperator::LessEq => IntCC::SignedLessThanOrEqual,
                                BinaryOperator::GreaterEq => IntCC::SignedGreaterThanOrEqual,
                                _ => IntCC::Equal,
                            };
                            let val = ctx.builder.ins().icmp(cc, lhs, rhs);
                            let true_val = ctx.builder.ins().iconst(types::I64, 1);
                            let false_val = ctx.builder.ins().iconst(types::I64, 0);
                            Ok(ctx.builder.ins().select(val, true_val, false_val))
                         }
                    },
                    BinaryOperator::In => {
                         // lhs = value to find
                         // rhs = collection (pointer to header)
                         // Return 1 if found, 0 if not
                         
                         let header_ptr = rhs;
                         let len = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 0);
                         let data_ptr = ctx.builder.ins().load(types::I64, MemFlags::new(), header_ptr, 8);
                         
                         let loop_block = ctx.builder.create_block();
                         let body_block = ctx.builder.create_block();
                         let next_block = ctx.builder.create_block();
                         let exit_block = ctx.builder.create_block();
                         
                         // Vars
                         let i_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                         ctx.builder.declare_var(i_var, types::I64);
                         let zero = ctx.builder.ins().iconst(types::I64, 0);
                         ctx.builder.def_var(i_var, zero);
                         
                         let result_var = Variable::new(*ctx.var_counter); *ctx.var_counter += 1;
                         ctx.builder.declare_var(result_var, types::I64);
                         let zero = ctx.builder.ins().iconst(types::I64, 0);
                         ctx.builder.def_var(result_var, zero);
                         
                         ctx.builder.ins().jump(loop_block, &[]);
                         
                         // LOOP CHECK
                         ctx.builder.switch_to_block(loop_block);
                         let i_val = ctx.builder.use_var(i_var);
                         let cmp_len = ctx.builder.ins().icmp(IntCC::SignedLessThan, i_val, len);
                         ctx.builder.ins().brif(cmp_len, body_block, &[], exit_block, &[]);
                         
                         // BODY
                         ctx.builder.switch_to_block(body_block);
                         let eight = ctx.builder.ins().iconst(types::I64, 8);
                         let offset = ctx.builder.ins().imul(i_val, eight);
                         let elem_addr = ctx.builder.ins().iadd(data_ptr, offset);
                         let elem_val = ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0);
                         
                         // Compare elem_val with lhs (target)
                         let is_eq = if matches!(left.ty, AstType::String) {
                             let strcmp_func = ctx.module.declare_func_in_func(ctx.strcmp_id, ctx.builder.func);
                             let call = ctx.builder.ins().call(strcmp_func, &[elem_val, lhs]);
                             let res = ctx.builder.inst_results(call)[0];
                             ctx.builder.ins().icmp_imm(IntCC::Equal, res, 0)
                         } else {
                             // Default to int comparison
                             ctx.builder.ins().icmp(IntCC::Equal, elem_val, lhs)
                         };
                         
                         let found_block = ctx.builder.create_block();
                         ctx.builder.ins().brif(is_eq, found_block, &[], next_block, &[]);
                         
                         // FOUND
                         ctx.builder.switch_to_block(found_block);
                         let one = ctx.builder.ins().iconst(types::I64, 1);
                         ctx.builder.def_var(result_var, one);
                         ctx.builder.ins().jump(exit_block, &[]);
                         
                         // NEXT
                         ctx.builder.switch_to_block(next_block);
                         let next_i = ctx.builder.ins().iadd_imm(i_val, 1);
                         ctx.builder.def_var(i_var, next_i);
                         ctx.builder.ins().jump(loop_block, &[]);
                         
                         // EXIT
                         ctx.builder.switch_to_block(exit_block);
                         ctx.builder.seal_block(loop_block);
                         ctx.builder.seal_block(body_block);
                         ctx.builder.seal_block(found_block);
                         ctx.builder.seal_block(next_block);
                         ctx.builder.seal_block(exit_block);
                         
                         Ok(ctx.builder.use_var(result_var))
                    }
                }
            }
        }
    }

}
