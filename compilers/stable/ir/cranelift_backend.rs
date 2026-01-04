// Cranelift Backend for GUL Stable Compiler
// Generates native machine code via Cranelift

use crate::ast::nodes::*;
use anyhow::{anyhow, Result};
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module, FuncId, DataId, DataDescription};
use cranelift::codegen;
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use std::collections::HashMap;

/// Cranelift code generator for GUL programs
pub struct CraneliftBackend {
    builder_context: FunctionBuilderContext,
    data_description: DataDescription,
}

/// Context passed during code generation
struct GenContext<'a, 'b> {
    builder: &'a mut FunctionBuilder<'b>,
    module: &'a mut ObjectModule,
    variables: &'a mut HashMap<String, Variable>,
    var_counter: &'a mut usize,
    printf_id: FuncId,
    gul_print_float_id: FuncId,
    format_int_id: DataId,
    format_str_id: DataId,
    string_literals: &'a mut HashMap<String, DataId>,
    data_description: &'a mut DataDescription,
}

impl CraneliftBackend {
    pub fn new() -> Self {
        CraneliftBackend {
            builder_context: FunctionBuilderContext::new(),
            data_description: DataDescription::new(),
        }
    }

    /// Generate object code from a GUL Program
    pub fn generate(&mut self, program: &Program) -> Result<Vec<u8>> {
        // Setup ISA for host target
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
        
        // Declare external functions
        let mut sig_printf = module.make_signature();
        sig_printf.params.push(AbiParam::new(types::I64)); 
        sig_printf.params.push(AbiParam::new(types::I64)); 
        sig_printf.returns.push(AbiParam::new(types::I32));
        let printf_id = module.declare_function("printf", Linkage::Import, &sig_printf)
            .map_err(|e| anyhow!("Failed to declare printf: {}", e))?;

        // Declare gul_print_float
        let mut sig_pf = module.make_signature();
        sig_pf.params.push(AbiParam::new(types::F64));
        sig_pf.returns.push(AbiParam::new(types::I32));
        let gul_print_float_id = module.declare_function("gul_print_float", Linkage::Import, &sig_pf)
            .map_err(|e| anyhow!("Failed to declare gul_print_float: {}", e))?;

        // Declare format strings
        self.data_description.define(b"%ld\n\0".to_vec().into_boxed_slice());
        let format_int_id = module.declare_data("fmt_int", Linkage::Local, true, false)
            .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_int_id, &self.data_description)
            .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        self.data_description.define(b"%s\n\0".to_vec().into_boxed_slice());
        let format_str_id = module.declare_data("fmt_str", Linkage::Local, true, false)
            .map_err(|e| anyhow!("Failed to declare data: {}", e))?;
        module.define_data(format_str_id, &self.data_description)
            .map_err(|e| anyhow!("Failed to define data: {}", e))?;
        self.data_description.clear();

        let mut ctx = codegen::Context::new();
        let mut string_literals = HashMap::new();
        
        // Generate main function from program's main_entry and statements
        self.generate_main(
            program,
            &mut module,
            &mut ctx,
            printf_id,
            gul_print_float_id,
            format_int_id,
            format_str_id,
            &mut string_literals,
        )?;
        
        let product = module.finish();
        Ok(product.emit().map_err(|e| anyhow!("Failed to emit object: {}", e))?)
    }

    fn generate_main(
        &mut self,
        program: &Program,
        module: &mut ObjectModule,
        ctx: &mut codegen::Context,
        printf_id: FuncId,
        gul_print_float_id: FuncId,
        format_int_id: DataId,
        format_str_id: DataId,
        string_literals: &mut HashMap<String, DataId>,
    ) -> Result<()> {
        // Create main function signature
        let mut sig = module.make_signature();
        sig.returns.push(AbiParam::new(types::I32)); // int main()
        
        let func_id = module.declare_function("main", Linkage::Export, &sig)
            .map_err(|e| anyhow!("Failed to declare main: {}", e))?;
        
        ctx.func.signature = sig;
        
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        
        let mut variables = HashMap::new();
        let mut var_counter = 0;
        
        {
            let mut gen_ctx = GenContext {
                builder: &mut builder,
                module,
                variables: &mut variables,
                var_counter: &mut var_counter,
                printf_id,
                gul_print_float_id,
                format_int_id,
                format_str_id,
                string_literals,
                data_description: &mut self.data_description,
            };

            // Generate code for main_entry statements
            for stmt in &program.main_entry {
                Self::generate_statement(stmt, &mut gen_ctx)?;
            }
            
            // Generate code for top-level statements
            for stmt in &program.statements {
                Self::generate_statement(stmt, &mut gen_ctx)?;
            }
            
            // Return 0
            let zero = gen_ctx.builder.ins().iconst(types::I32, 0);
            gen_ctx.builder.ins().return_(&[zero]);
        }
        
        builder.finalize();
        
        module.define_function(func_id, ctx)
            .map_err(|e| anyhow!("Failed to define main: {}", e))?;
        module.clear_context(ctx);
        
        Ok(())
    }

    fn generate_statement(stmt: &Statement, ctx: &mut GenContext) -> Result<()> {
        match stmt {
            Statement::LetDecl(let_stmt) => {
                let val = Self::generate_expression(&let_stmt.value, ctx)?;
                let var = Variable::new(*ctx.var_counter);
                *ctx.var_counter += 1;
                ctx.builder.declare_var(var, types::I64);
                ctx.builder.def_var(var, val);
                ctx.variables.insert(let_stmt.name.clone(), var);
            }
            Statement::VarDecl(var_stmt) => {
                let val = Self::generate_expression(&var_stmt.value, ctx)?;
                let var = Variable::new(*ctx.var_counter);
                *ctx.var_counter += 1;
                ctx.builder.declare_var(var, types::I64);
                ctx.builder.def_var(var, val);
                ctx.variables.insert(var_stmt.name.clone(), var);
            }
            Statement::AssignmentStmt(assign) => {
                let val = Self::generate_expression(&assign.value, ctx)?;
                // Extract variable name from target expression (if identifier)
                if let Expression::Identifier(ident) = &assign.target {
                    if let Some(var) = ctx.variables.get(&ident.name) {
                        ctx.builder.def_var(*var, val);
                    }
                }
            }
            Statement::ExpressionStmt(expr_stmt) => {
                Self::generate_expression(&expr_stmt.expression, ctx)?;
            }
            Statement::WhileStmt(while_stmt) => {
                let header = ctx.builder.create_block();
                let body_block = ctx.builder.create_block();
                let exit = ctx.builder.create_block();
                
                ctx.builder.ins().jump(header, &[]);
                ctx.builder.switch_to_block(header);
                
                let cond_val = Self::generate_expression(&while_stmt.condition, ctx)?;
                ctx.builder.ins().brif(cond_val, body_block, &[], exit, &[]);
                
                ctx.builder.switch_to_block(body_block);
                for s in &while_stmt.body {
                    Self::generate_statement(s, ctx)?;
                }
                ctx.builder.ins().jump(header, &[]);
                
                ctx.builder.switch_to_block(exit);
                ctx.builder.seal_block(header);
                ctx.builder.seal_block(body_block);
                ctx.builder.seal_block(exit);
            }
            Statement::IfStmt(if_stmt) => {
                let then_block = ctx.builder.create_block();
                let else_block = ctx.builder.create_block();
                let merge_block = ctx.builder.create_block();
                
                let cond_val = Self::generate_expression(&if_stmt.condition, ctx)?;
                ctx.builder.ins().brif(cond_val, then_block, &[], else_block, &[]);
                
                ctx.builder.switch_to_block(then_block);
                ctx.builder.seal_block(then_block);
                for s in &if_stmt.then_body {
                    Self::generate_statement(s, ctx)?;
                }
                ctx.builder.ins().jump(merge_block, &[]);
                
                ctx.builder.switch_to_block(else_block);
                ctx.builder.seal_block(else_block);
                for s in &if_stmt.else_body {
                    Self::generate_statement(s, ctx)?;
                }
                ctx.builder.ins().jump(merge_block, &[]);
                
                ctx.builder.switch_to_block(merge_block);
                ctx.builder.seal_block(merge_block);
            }
            _ => {} // Skip unsupported statements for now
        }
        Ok(())
    }

    fn generate_expression(expr: &Expression, ctx: &mut GenContext) -> Result<Value> {
        match expr {
            Expression::Literal(lit) => {
                match lit.value_type {
                    crate::lexer::token::TokenType::Integer => {
                        let n: i64 = lit.value.parse().unwrap_or(0);
                        Ok(ctx.builder.ins().iconst(types::I64, n))
                    }
                    crate::lexer::token::TokenType::Float => {
                        let f: f64 = lit.value.parse().unwrap_or(0.0);
                        Ok(ctx.builder.ins().f64const(f))
                    }
                    crate::lexer::token::TokenType::String => {
                        // Create string literal data
                        let data_id = if let Some(id) = ctx.string_literals.get(&lit.value) {
                            *id
                        } else {
                            let mut bytes = lit.value.as_bytes().to_vec();
                            bytes.push(0); // Null terminate
                            ctx.data_description.define(bytes.into_boxed_slice());
                            let id = ctx.module.declare_data(
                                &format!("str_{}", ctx.string_literals.len()),
                                Linkage::Local, true, false
                            ).map_err(|e| anyhow!("Failed to declare string: {}", e))?;
                            ctx.module.define_data(id, ctx.data_description)
                                .map_err(|e| anyhow!("Failed to define string: {}", e))?;
                            ctx.data_description.clear();
                            ctx.string_literals.insert(lit.value.clone(), id);
                            id
                        };
                        let data_val = ctx.module.declare_data_in_func(data_id, ctx.builder.func);
                        Ok(ctx.builder.ins().global_value(types::I64, data_val))
                    }
                    crate::lexer::token::TokenType::TrueKeyword => {
                        Ok(ctx.builder.ins().iconst(types::I64, 1))
                    }
                    crate::lexer::token::TokenType::FalseKeyword => {
                        Ok(ctx.builder.ins().iconst(types::I64, 0))
                    }
                    _ => Ok(ctx.builder.ins().iconst(types::I64, 0))
                }
            }
            Expression::Identifier(ident) => {
                if let Some(var) = ctx.variables.get(&ident.name) {
                    Ok(ctx.builder.use_var(*var))
                } else {
                    Ok(ctx.builder.ins().iconst(types::I64, 0))
                }
            }
            Expression::BinaryOp(binop) => {
                let left = Self::generate_expression(&binop.left, ctx)?;
                let right = Self::generate_expression(&binop.right, ctx)?;
                
                use crate::lexer::token::TokenType;
                match binop.operator {
                    TokenType::Plus => Ok(ctx.builder.ins().iadd(left, right)),
                    TokenType::Minus => Ok(ctx.builder.ins().isub(left, right)),
                    TokenType::Star => Ok(ctx.builder.ins().imul(left, right)),
                    TokenType::Slash => Ok(ctx.builder.ins().sdiv(left, right)),
                    TokenType::Greater => Ok(ctx.builder.ins().icmp(IntCC::SignedGreaterThan, left, right)),
                    TokenType::Less => Ok(ctx.builder.ins().icmp(IntCC::SignedLessThan, left, right)),
                    TokenType::GreaterEq => Ok(ctx.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, left, right)),
                    TokenType::LessEq => Ok(ctx.builder.ins().icmp(IntCC::SignedLessThanOrEqual, left, right)),
                    TokenType::EqualEqual => Ok(ctx.builder.ins().icmp(IntCC::Equal, left, right)),
                    TokenType::NotEqual => Ok(ctx.builder.ins().icmp(IntCC::NotEqual, left, right)),
                    _ => Ok(ctx.builder.ins().iconst(types::I64, 0))
                }
            }
            Expression::Call(call) => {
                // Handle print specially
                if let Expression::Identifier(ident) = call.callee.as_ref() {
                    if ident.name == "print" && !call.arguments.is_empty() {
                        let arg = Self::generate_expression(&call.arguments[0], ctx)?;
                        
                        // Get printf reference
                        let printf_ref = ctx.module.declare_func_in_func(ctx.printf_id, ctx.builder.func);
                        
                        // Determine format based on argument type
                        let fmt_id = ctx.format_int_id;
                        let fmt_val = ctx.module.declare_data_in_func(fmt_id, ctx.builder.func);
                        let fmt_ptr = ctx.builder.ins().global_value(types::I64, fmt_val);
                        
                        let call_result = ctx.builder.ins().call(printf_ref, &[fmt_ptr, arg]);
                        return Ok(ctx.builder.inst_results(call_result)[0]);
                    }
                }
                Ok(ctx.builder.ins().iconst(types::I64, 0))
            }
            _ => Ok(ctx.builder.ins().iconst(types::I64, 0))
        }
    }
}
