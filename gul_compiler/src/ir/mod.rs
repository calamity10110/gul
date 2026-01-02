// IR Generator using Cranelift

use crate::ast::{Program, Statement, Expression, ExpressionKind, BinaryOperator, Type as AstType, Pattern};
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
    gul_concat_id: FuncId,
    gul_int_to_str_id: FuncId,
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
                    printf_id, malloc_id, gul_concat_id, gul_int_to_str_id, strcmp_id,
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
        _params: &[String],
        body: &[Statement],
        module: &mut ObjectModule,
        ctx: &mut codegen::Context,
        printf_id: FuncId,
        malloc_id: FuncId,
        gul_concat_id: FuncId,
        gul_int_to_str_id: FuncId,
        strcmp_id: FuncId,
        format_int_id: DataId,
        format_str_id: DataId,
        format_true_id: DataId,
        format_false_id: DataId,
        string_literals: &mut HashMap<String, DataId>,
    ) -> Result<()> {
        let mut sig = module.make_signature();
        sig.returns.push(AbiParam::new(types::I64));
        
        let func_id = module
            .declare_function(name, Linkage::Export, &sig)
            .map_err(|e| anyhow!("Failed to declare function: {}", e))?;
        
        ctx.func.signature = sig;
        
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        
        let mut loop_stack = Vec::new();
        let mut variables = HashMap::new();
        let mut var_counter = 0;
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
                gul_concat_id,
                gul_int_to_str_id,
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
            Statement::Let { name, value } | Statement::Var { name, value } => {
                let val = Self::generate_expression(value, ctx)?;
                
                let var = if let Some(existing_var) = ctx.variables.get(name) {
                    *existing_var
                } else {
                    let var = Variable::new(*ctx.var_counter);
                    *ctx.var_counter += 1;
                    ctx.variables.insert(name.clone(), var);
                    ctx.builder.declare_var(var, types::I64);
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
                let base_ptr = Self::generate_expression(target, ctx)?;
                let idx = Self::generate_expression(index, ctx)?;
                let val = Self::generate_expression(value, ctx)?;
                
                let eight = ctx.builder.ins().iconst(types::I64, 8);
                let offset_mul = ctx.builder.ins().imul(idx, eight);
                let base_adj = ctx.builder.ins().iadd_imm(base_ptr, 8);
                let elem_addr = ctx.builder.ins().iadd(base_adj, offset_mul);
                
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
                                      let pat_val = ctx.builder.ins().iconst(types::I64, *n);
                                      let cmp = ctx.builder.ins().icmp(IntCC::Equal, val, pat_val);
                                      ctx.builder.ins().brif(cmp, body_block, &[], next_check_block, &[]);
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
                         ctx.builder.declare_var(var, types::I64);
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
                let size_bytes = (len + 1) * 8;
                let size_val = ctx.builder.ins().iconst(types::I64, size_bytes as i64);
                
                let malloc_func = ctx.module.declare_func_in_func(ctx.malloc_id, ctx.builder.func);
                let call = ctx.builder.ins().call(malloc_func, &[size_val]);
                let base_ptr = ctx.builder.inst_results(call)[0];
                
                // Store length
                let len_val = ctx.builder.ins().iconst(types::I64, len as i64);
                // store: value, base, offset
                ctx.builder.ins().store(MemFlags::new(), len_val, base_ptr, 0);
                
                for (i, item) in items.iter().enumerate() {
                    let val = Self::generate_expression(item, ctx)?;
                    let offset = ((i + 1) * 8) as i32;
                    ctx.builder.ins().store(MemFlags::new(), val, base_ptr, offset);
                }
                
                Ok(base_ptr)
            },
            ExpressionKind::Index { target, index } => {
                let base_ptr = Self::generate_expression(target, ctx)?;
                let idx = Self::generate_expression(index, ctx)?;
                
                // Address = base + 8 + (idx * 8)
                let eight = ctx.builder.ins().iconst(types::I64, 8);
                let offset_mul = ctx.builder.ins().imul(idx, eight);
                let base_adj = ctx.builder.ins().iadd_imm(base_ptr, 8);
                let elem_addr = ctx.builder.ins().iadd(base_adj, offset_mul);
                
                Ok(ctx.builder.ins().load(types::I64, MemFlags::new(), elem_addr, 0))
            },
            ExpressionKind::Call { callee, args } => {
                if let ExpressionKind::Identifier(name) = &callee.kind {
                    if name == "print" {
                        if args.len() != 1 {
                            return Err(anyhow!("print() expects 1 argument"));
                        }
                        let arg_expr = &args[0];
                        let arg_val = Self::generate_expression(arg_expr, ctx)?;
                        
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
                    }
                }
                Err(anyhow!("Function calls not fully implemented except 'print'."))
            },
            ExpressionKind::BinaryOp { left, op, right } => {
                let lhs = Self::generate_expression(left, ctx)?;
                let rhs = Self::generate_expression(right, ctx)?;
                match op {
                    BinaryOperator::Add => {
                        if matches!(left.ty, AstType::String) || matches!(right.ty, AstType::String) {
                            let l_val = if matches!(left.ty, AstType::String) {
                                lhs
                            } else {
                                let i2s = ctx.module.declare_func_in_func(ctx.gul_int_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(i2s, &[lhs]);
                                ctx.builder.inst_results(call)[0]
                            };
                            
                            let r_val = if matches!(right.ty, AstType::String) {
                                rhs
                            } else {
                                let i2s = ctx.module.declare_func_in_func(ctx.gul_int_to_str_id, ctx.builder.func);
                                let call = ctx.builder.ins().call(i2s, &[rhs]);
                                ctx.builder.inst_results(call)[0]
                            };
                            
                            let concat = ctx.module.declare_func_in_func(ctx.gul_concat_id, ctx.builder.func);
                            let call = ctx.builder.ins().call(concat, &[l_val, r_val]);
                            Ok(ctx.builder.inst_results(call)[0])
                        } else {
                            Ok(ctx.builder.ins().iadd(lhs, rhs))
                        }
                    },
                    BinaryOperator::Subtract => Ok(ctx.builder.ins().isub(lhs, rhs)),
                    BinaryOperator::Multiply => Ok(ctx.builder.ins().imul(lhs, rhs)),
                    BinaryOperator::Divide => Ok(ctx.builder.ins().sdiv(lhs, rhs)),
                    BinaryOperator::Equal | BinaryOperator::NotEqual | 
                    BinaryOperator::Less | BinaryOperator::Greater |
                    BinaryOperator::LessEq | BinaryOperator::GreaterEq => {
                         let cmp = match op {
                            BinaryOperator::Equal => ctx.builder.ins().icmp(IntCC::Equal, lhs, rhs),
                            BinaryOperator::NotEqual => ctx.builder.ins().icmp(IntCC::NotEqual, lhs, rhs),
                            BinaryOperator::Less => ctx.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs),
                            BinaryOperator::Greater => ctx.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs),
                            BinaryOperator::LessEq => ctx.builder.ins().icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
                            BinaryOperator::GreaterEq => ctx.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
                            _ => unreachable!(),
                        };
                         let true_val = ctx.builder.ins().iconst(types::I64, 1);
                         let false_val = ctx.builder.ins().iconst(types::I64, 0);
                         Ok(ctx.builder.ins().select(cmp, true_val, false_val))
                    },
                }
            }
        }
    }
}
