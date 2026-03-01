// Cranelift Backend for GUL Stable Compiler
// Generates native machine code via Cranelift

use crate::ast::nodes::*;
use crate::lexer::token::TokenType;
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
    variables: &'a mut HashMap<String, (Variable, String)>, // name -> (variable reference, type)
    var_counter: &'a mut usize,
    printf_id: FuncId,
    gul_print_float_id: FuncId,
    gul_input_str_id: FuncId,
    gul_input_int_id: FuncId,
    gul_input_flt_id: FuncId,
    gul_table_alloc_id: FuncId,
    gul_table_set_col_name_id: FuncId,
    gul_table_set_row_id: FuncId,
    gul_malloc_id: FuncId,
    gul_list_alloc_id: FuncId,
    gul_list_push_id: FuncId,
    gul_dict_alloc_id: FuncId,
    gul_dict_set_id: FuncId,
    gul_set_alloc_id: FuncId,
    gul_set_add_id: FuncId,
    gul_list_get_id: FuncId,
    gul_list_len_id: FuncId,
    gul_dict_get_id: FuncId,
    gul_int_to_string_id: FuncId,
    gul_float_to_string_id: FuncId,
    gul_string_concat_id: FuncId,
    format_int_id: DataId,
    format_str_id: DataId,
    string_literals: &'a mut HashMap<String, DataId>,
    data_description: &'a mut DataDescription,
    builtins: HashMap<String, FuncId>,
    functions: &'a HashMap<String, FuncId>,
    struct_layouts: &'a mut HashMap<String, HashMap<String, usize>>,
    current_func_name: String,
}

impl CraneliftBackend {
    pub fn new() -> Self {
        CraneliftBackend {
            builder_context: FunctionBuilderContext::new(),
            data_description: DataDescription::new(),
        }
    }

    fn generate_string_literal(text: &str, ctx: &mut GenContext) -> Result<Value> {
         // Helper to create string literal value
         let data_id = if let Some(id) = ctx.string_literals.get(text) {
             *id
         } else {
             let mut bytes = text.as_bytes().to_vec();
             bytes.push(0); // Null terminate
             ctx.data_description.define(bytes.into_boxed_slice());
             let id = ctx.module.declare_data(
                 &format!("str_{}", ctx.string_literals.len()),
                 Linkage::Local, true, false
             ).map_err(|e| anyhow!("Failed to declare string data: {}", e))?;
             
             ctx.module.define_data(id, ctx.data_description)
                 .map_err(|e| anyhow!("Failed to define string data: {}", e))?;
             
             ctx.data_description.clear();
             ctx.string_literals.insert(text.to_string(), id);
             id
         };
         
         let sym = ctx.module.declare_data_in_func(data_id, ctx.builder.func);
         Ok(ctx.builder.ins().symbol_value(types::I64, sym))
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

        // Declare gul_input_str() -> i64 (char*)
        let mut sig_input_str = module.make_signature();
        sig_input_str.returns.push(AbiParam::new(types::I64));
        let gul_input_str_id = module.declare_function("gul_input_str", Linkage::Import, &sig_input_str)
            .map_err(|e| anyhow!("Failed to declare gul_input_str: {}", e))?;

        let mut sig_table_alloc = module.make_signature();
        sig_table_alloc.params.push(AbiParam::new(types::I64)); // col_count
        sig_table_alloc.params.push(AbiParam::new(types::I64)); // row_count
        sig_table_alloc.returns.push(AbiParam::new(types::I64)); // ptr
        let gul_table_alloc_id = module.declare_function("gul_table_alloc", Linkage::Import, &sig_table_alloc)
            .map_err(|e| anyhow!("Failed to declare gul_table_alloc: {}", e))?;

        let mut sig_tbl_col = module.make_signature();
        sig_tbl_col.params.push(AbiParam::new(types::I64)); // table_ptr
        sig_tbl_col.params.push(AbiParam::new(types::I64)); // idx
        sig_tbl_col.params.push(AbiParam::new(types::I64)); // name_ptr
        let gul_table_set_col_name_id = module.declare_function("gul_table_set_col_name", Linkage::Import, &sig_tbl_col)
            .map_err(|e| anyhow!("Failed to declare gul_table_set_col_name: {}", e))?;

        let mut sig_tbl_row = module.make_signature();
        sig_tbl_row.params.push(AbiParam::new(types::I64)); // table_ptr
        sig_tbl_row.params.push(AbiParam::new(types::I64)); // idx
        sig_tbl_row.params.push(AbiParam::new(types::I64)); // name_ptr
        sig_tbl_row.params.push(AbiParam::new(types::I64)); // val_ptr
        let gul_table_set_row_id = module.declare_function("gul_table_set_row", Linkage::Import, &sig_tbl_row)
            .map_err(|e| anyhow!("Failed to declare gul_table_set_row: {}", e))?;

        let mut sig_malloc = module.make_signature();
        sig_malloc.params.push(AbiParam::new(types::I64));
        sig_malloc.returns.push(AbiParam::new(types::I64));
        let gul_malloc_id = module.declare_function("gul_malloc", Linkage::Import, &sig_malloc)
            .map_err(|e| anyhow!("Failed to declare gul_malloc: {}", e))?;

        // Declare gul_input_int() -> i64
        let mut sig_input_int = module.make_signature();
        sig_input_int.returns.push(AbiParam::new(types::I64));
        let gul_input_int_id = module.declare_function("gul_input_int", Linkage::Import, &sig_input_int)
            .map_err(|e| anyhow!("Failed to declare gul_input_int: {}", e))?;

        // Declare gul_input_flt() -> f64
        let mut sig_input_flt = module.make_signature();
        sig_input_flt.returns.push(AbiParam::new(types::F64));
        let gul_input_flt_id = module.declare_function("gul_input_flt", Linkage::Import, &sig_input_flt)
            .map_err(|e| anyhow!("Failed to declare gul_input_flt: {}", e))?;

        // Declare gul_string_concat(i64, i64) -> i64
        let mut sig_concat = module.make_signature();
        sig_concat.params.push(AbiParam::new(types::I64));
        sig_concat.params.push(AbiParam::new(types::I64));
        sig_concat.returns.push(AbiParam::new(types::I64));
        let gul_string_concat_id = module.declare_function("gul_string_concat", Linkage::Import, &sig_concat)
            .map_err(|e| anyhow!("Failed to declare gul_string_concat: {}", e))?;

        // Declare gul_list_alloc(capacity) -> list_ptr
        let mut sig_list_alloc = module.make_signature();
        sig_list_alloc.params.push(AbiParam::new(types::I64));
        sig_list_alloc.returns.push(AbiParam::new(types::I64));
        let gul_list_alloc_id = module.declare_function("gul_list_alloc", Linkage::Import, &sig_list_alloc)
            .map_err(|e| anyhow!("Failed to declare gul_list_alloc: {}", e))?;

        // Declare gul_list_push(list_ptr, value)
        let mut sig_list_push = module.make_signature();
        sig_list_push.params.push(AbiParam::new(types::I64));
        sig_list_push.params.push(AbiParam::new(types::I64));
        let gul_list_push_id = module.declare_function("gul_list_push", Linkage::Import, &sig_list_push)
            .map_err(|e| anyhow!("Failed to declare gul_list_push: {}", e))?;
        
        // Declare gul_list_get(list_ptr, index) -> value
        let mut sig_list_get = module.make_signature();
        sig_list_get.params.push(AbiParam::new(types::I64)); // list_ptr
        sig_list_get.params.push(AbiParam::new(types::I64)); // index
        sig_list_get.returns.push(AbiParam::new(types::I64)); // value
        let gul_list_get_id = module.declare_function("gul_list_get", Linkage::Import, &sig_list_get)
            .map_err(|e| anyhow!("Failed to declare gul_list_get: {}", e))?;

        // Declare gul_list_len(list_ptr) -> i64
        let mut sig_list_len = module.make_signature();
        sig_list_len.params.push(AbiParam::new(types::I64));
        sig_list_len.returns.push(AbiParam::new(types::I64));
        let gul_list_len_id = module.declare_function("gul_list_len", Linkage::Import, &sig_list_len)
            .map_err(|e| anyhow!("Failed to declare gul_list_len: {}", e))?;

        // Declare gul_dict_alloc(capacity) -> dict_ptr
        let mut sig_dict_alloc = module.make_signature();
        sig_dict_alloc.params.push(AbiParam::new(types::I64));
        sig_dict_alloc.returns.push(AbiParam::new(types::I64));
        let gul_dict_alloc_id = module.declare_function("gul_dict_alloc", Linkage::Import, &sig_dict_alloc)
            .map_err(|e| anyhow!("Failed to declare gul_dict_alloc: {}", e))?;

        // Declare gul_dict_set(dict_ptr, key_ptr, value)
        let mut sig_dict_set = module.make_signature();
        sig_dict_set.params.push(AbiParam::new(types::I64));
        sig_dict_set.params.push(AbiParam::new(types::I64));
        sig_dict_set.params.push(AbiParam::new(types::I64));
        let gul_dict_set_id = module.declare_function("gul_dict_set", Linkage::Import, &sig_dict_set)
            .map_err(|e| anyhow!("Failed to declare gul_dict_set: {}", e))?;

        // Declare gul_dict_get(dict_ptr, key_ptr) -> value
        let mut sig_dict_get = module.make_signature();
        sig_dict_get.params.push(AbiParam::new(types::I64)); // dict_ptr
        sig_dict_get.params.push(AbiParam::new(types::I64)); // key_ptr
        sig_dict_get.returns.push(AbiParam::new(types::I64)); // value
        let gul_dict_get_id = module.declare_function("gul_dict_get", Linkage::Import, &sig_dict_get)
            .map_err(|e| anyhow!("Failed to declare gul_dict_get: {}", e))?;

        // gul_int_to_string(int) -> str
        let mut sig_int_to_str = module.make_signature();
        sig_int_to_str.params.push(AbiParam::new(types::I64));
        sig_int_to_str.returns.push(AbiParam::new(types::I64));
        let gul_int_to_string_id = module.declare_function("gul_int_to_string", Linkage::Import, &sig_int_to_str)
            .map_err(|e| anyhow!("Failed to declare gul_int_to_string: {}", e))?;

        // gul_float_to_string(float) -> str
        let mut sig_flt_to_str = module.make_signature();
        sig_flt_to_str.params.push(AbiParam::new(types::F64));
        sig_flt_to_str.returns.push(AbiParam::new(types::I64));
        let gul_float_to_string_id = module.declare_function("gul_float_to_string", Linkage::Import, &sig_flt_to_str)
            .map_err(|e| anyhow!("Failed to declare gul_float_to_string: {}", e))?;

        // Declare gul_set_alloc(capacity) -> set_ptr
        let mut sig_set_alloc = module.make_signature();
        sig_set_alloc.params.push(AbiParam::new(types::I64));
        sig_set_alloc.returns.push(AbiParam::new(types::I64));
        let gul_set_alloc_id = module.declare_function("gul_set_alloc", Linkage::Import, &sig_set_alloc)
            .map_err(|e| anyhow!("Failed to declare gul_set_alloc: {}", e))?;

        // Declare gul_set_add(set_ptr, value)
        let mut sig_set_add = module.make_signature();
        sig_set_add.params.push(AbiParam::new(types::I64));
        sig_set_add.params.push(AbiParam::new(types::I64));
        let gul_set_add_id = module.declare_function("gul_set_add", Linkage::Import, &sig_set_add)
            .map_err(|e| anyhow!("Failed to declare gul_set_add: {}", e))?;

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
        
        // Declare Generic Builtins
        let mut builtins = HashMap::new();
        let builtin_names = vec![
            "gul_math_sin", "gul_math_cos", "gul_math_exp", "gul_math_log",
            // "gul_print_float", "gul_input_int", "gul_input_flt", "gul_input_str", // Handled explicitly
            "gul_math_tan", "gul_math_asin", "gul_math_acos", "gul_math_atan", "gul_math_atan2",
            "gul_math_exp", "gul_math_log", "gul_math_log10", "gul_math_log2", "gul_math_pow", "gul_math_sqrt", "gul_math_cbrt",
            "gul_math_floor", "gul_math_ceil", "gul_math_round", "gul_math_trunc", "gul_math_abs", "gul_math_min", "gul_math_max",
            "gul_ml_sigmoid", "gul_ml_tanh", "gul_ml_relu",
            "gul_tensor_alloc", "gul_tensor_free", "gul_tensor_fill", 
            "gul_tensor_add", "gul_tensor_mul", "gul_tensor_matmul", 
            "gul_tensor_sum", "gul_tensor_mean",
            "gul_file_open", "gul_file_close",            "gul_file_read_line",
            // String
            "gul_string_substr",
            "gul_string_get",
            "gul_exec_foreign",
            // gul_malloc handled explicitly
            // Autograd
            "gul_autograd_begin", "gul_autograd_end",
            "gul_make_var", "gul_var_val", "gul_var_grad",
            "gul_var_add", "gul_var_mul", "gul_var_sin", "gul_backward"
        ];
        
        for name in builtin_names {
             let mut sig = module.make_signature();
             // Note: All our runtime functions use I64 or F64 mostly.
             // For simplicity, we declare them varargs-like or just trust the call signature from GUL matches?
             // Cranelift requires specific signature.
             // But stdlib.c functions are: double(double) or int64(int64).
             // To handle this properly, we need specific signatures.
             // HACK: For now, we assume user GUL code provides correct args, but Cranelift needs types for declaration?
             // Actually, Call instruction verification relies on declared sig.
             // If we get it wrong, it might crash.
             // We can use a helper to guess signature based on name?
             // Or just make them all take ...? No, C ABI.
             
             // Simplification: We only support the specific ones we know.
             if name.contains("math") || name.contains("ml") {
                 // double -> double (mostly)
                 // pow, atan2, log_base, min, max (2 args)
                 if name.ends_with("pow") || name.ends_with("atan2") || name.ends_with("min") || name.ends_with("max") {
                     sig.params.push(AbiParam::new(types::F64));
                     sig.params.push(AbiParam::new(types::F64));
                 } else {
                     sig.params.push(AbiParam::new(types::F64));
                 }
                 sig.returns.push(AbiParam::new(types::F64));
             } else if name.contains("tensor") {
                 // Tensor ops: alloc(i64)->i64, free(i64), fill(i64,i64,f64), add(i64,i64,i64,i64), matmul(i64...)
                 if name.ends_with("alloc") {
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::I64));
                 } else if name.ends_with("free") {
                     sig.params.push(AbiParam::new(types::I64));
                 } else if name.ends_with("fill") { // ptr, size, val
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::F64));
                 } else if name.ends_with("add") || name.ends_with("mul") { // dst, a, b, size
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                 } else if name.ends_with("matmul") { // c, a, b, m, k, n
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                 } else if name.ends_with("sum") || name.ends_with("mean") { // ptr, size -> f64
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::F64));
                 }
             } else if name.contains("file") {
                 // open(i64, i64) -> i64, close(i64), readline(i64) -> i64
                 if name.ends_with("open") {
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::I64));
                 } else if name.ends_with("close") {
                     sig.params.push(AbiParam::new(types::I64));
                 } else if name.ends_with("read_line") {
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::I64));
                 }
             } else if name.contains("autograd") || name.contains("var") || name.contains("backward") {
                 if name.ends_with("begin") || name.ends_with("end") {
                     // takes nothing
                 } else if name.ends_with("make_var") {
                     sig.params.push(AbiParam::new(types::F64));
                     sig.returns.push(AbiParam::new(types::I64));
                 } else if name.ends_with("val") || name.ends_with("grad") {
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::F64));
                 } else if name.ends_with("add") || name.ends_with("mul") {
                     sig.params.push(AbiParam::new(types::I64));
                     sig.params.push(AbiParam::new(types::I64));
                     sig.returns.push(AbiParam::new(types::I64));
                 } else if name.ends_with("sin") || name.ends_with("backward") {
                     sig.params.push(AbiParam::new(types::I64));
                     if name.ends_with("sin") { sig.returns.push(AbiParam::new(types::I64)); }
                 }
             }
             
             let fid = module.declare_function(name, Linkage::Import, &sig)
                 .map_err(|e| anyhow!("Failed to declare {}: {}", name, e))?;
             builtins.insert(name.to_string(), fid);
             builtins.insert(name.replace("gul_", ""), fid);
        }

        let mut functions = HashMap::new();
        // 1. Declare all user functions
        for func in &program.functions {
            let mut sig = module.make_signature();
            // Params
            for param in &func.parameters {
                // Map GUL types to Cranelift types. Default I64 for now unless inferred?
                // AST Parameter has type_annotation: Option<String>.
                let ty = if !param.type_annotation.is_empty() {
                    let t = &param.type_annotation;
                    if t == "int" { types::I64 }
                    else if t == "float" { types::F64 }
                    else if t == "str" { types::I64 } // pointer
                    else { types::I64 }
                } else {
                    types::I64
                };
                sig.params.push(AbiParam::new(ty));
            }
            // Return type
            if !func.return_type.is_empty() {
                 let ret_type = &func.return_type;
                 let ty = if ret_type == "int" { types::I64 }
                    else if ret_type == "float" { types::F64 }
                    else if ret_type == "str" { types::I64 }
                    else { types::I64 };
                 sig.returns.push(AbiParam::new(ty));
            } else {
                 // Void return? Or default 0?
                 sig.returns.push(AbiParam::new(types::I64)); // Default return 0
            }
            
            // Rename 'main' to avoid conflict with the CRT entry point
            let internal_name = if func.name == "main" { "gul_user_main".to_string() } else { func.name.clone() };
            let func_id = module.declare_function(&internal_name, Linkage::Local, &sig)
                .map_err(|e| anyhow!("Failed to declare function {}: {}", func.name, e))?;
            functions.insert(func.name.clone(), func_id);
        }

        // 2. Generate all user functions
        for func_def in &program.functions {
             let mut sub_ctx = codegen::Context::new();
             sub_ctx.func.signature = module.make_signature(); // Initialize signature container
             self.generate_function(
                func_def,
                &mut module,
                &mut sub_ctx,
                printf_id,
                gul_print_float_id,
                gul_input_str_id,
                gul_input_int_id,
                gul_input_flt_id,
                gul_table_alloc_id,
                gul_table_set_col_name_id,
                gul_table_set_row_id,
                gul_malloc_id,
                gul_list_alloc_id,
                gul_list_push_id,
                gul_dict_alloc_id,
                gul_dict_set_id,
                gul_set_alloc_id,
                gul_set_add_id,
                gul_list_get_id,
                gul_list_len_id,
                gul_dict_get_id,
                gul_int_to_string_id,
                gul_float_to_string_id,
                gul_string_concat_id,
                format_int_id,
                format_str_id,
                &mut string_literals,
                builtins.clone(),
                &functions,
            )?;
        }

        // Generate main function from program's main_entry and statements
        self.generate_main(
            program,
            &mut module,
            &mut ctx,
            printf_id,
            gul_print_float_id,
            gul_input_str_id,
            gul_input_int_id,
            gul_input_flt_id,
            gul_table_alloc_id,
            gul_table_set_col_name_id,
            gul_table_set_row_id,
            gul_malloc_id,
            gul_list_alloc_id,
            gul_list_push_id,
            gul_dict_alloc_id,
            gul_dict_set_id,
            gul_set_alloc_id,
            gul_set_add_id,
            gul_list_get_id,
            gul_list_len_id,
            gul_dict_get_id,
            gul_int_to_string_id,
            gul_float_to_string_id,
            gul_string_concat_id,
            format_int_id,
            format_str_id,
            &mut string_literals,
            builtins,
            &functions,
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
        gul_input_str_id: FuncId,
        gul_input_int_id: FuncId,
        gul_input_flt_id: FuncId,
        gul_table_alloc_id: FuncId,
        gul_table_set_col_name_id: FuncId,
        gul_table_set_row_id: FuncId,
        gul_malloc_id: FuncId,
        gul_list_alloc_id: FuncId,
        gul_list_push_id: FuncId,
        gul_dict_alloc_id: FuncId,
        gul_dict_set_id: FuncId,
        gul_set_alloc_id: FuncId,
        gul_set_add_id: FuncId,
        gul_list_get_id: FuncId,
        gul_list_len_id: FuncId,
        gul_dict_get_id: FuncId,
        gul_int_to_string_id: FuncId,
        gul_float_to_string_id: FuncId,
        gul_string_concat_id: FuncId,
        format_int_id: DataId,
        format_str_id: DataId,
        string_literals: &mut HashMap<String, DataId>,
        builtins: HashMap<String, FuncId>,
        functions: &HashMap<String, FuncId>,
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
        
        let mut variables: HashMap<String, (Variable, String)> = HashMap::new();
        let mut struct_layouts = HashMap::new();
        let mut var_counter = 0;
        
        {
            let mut gen_ctx = GenContext {
                builder: &mut builder,
                module,
                variables: &mut variables,
                var_counter: &mut var_counter,
                printf_id,
                gul_print_float_id,
                gul_input_str_id,
                gul_input_int_id,
                gul_input_flt_id,
                gul_table_alloc_id,
                gul_table_set_col_name_id,
                gul_table_set_row_id,
                gul_malloc_id,
                gul_list_alloc_id,
                gul_list_push_id,
                gul_dict_alloc_id,
                gul_dict_set_id,
                gul_set_alloc_id,
                gul_set_add_id,
                gul_list_get_id,
                gul_list_len_id,
                gul_dict_get_id,
                gul_int_to_string_id,
                gul_float_to_string_id,
                gul_string_concat_id,
                format_int_id,
                format_str_id,
                string_literals,
                data_description: &mut self.data_description,
                builtins,
                functions,
                struct_layouts: &mut struct_layouts,
                current_func_name: "main".to_string(),
            };
            
            // Generate code for main_entry statements
            for stmt in &program.main_entry {
                if Self::generate_statement(stmt, &mut gen_ctx)? { break; }
            }
            
            // If main_entry is empty, check if there is a 'main' function to call
            if program.main_entry.is_empty() {
                if let Some(fid) = functions.get("main") {
                    let func_ref = gen_ctx.module.declare_func_in_func(*fid, gen_ctx.builder.func);
                    gen_ctx.builder.ins().call(func_ref, &[]);
                }
            }
            
            // Generate code for top-level statements
            for stmt in &program.statements {
                if Self::generate_statement(stmt, &mut gen_ctx)? { break; }
            }
            
            // Return 0
            let current_block = gen_ctx.builder.current_block().unwrap();
            let is_terminated = if let Some(last_inst) = gen_ctx.builder.func.layout.last_inst(current_block) {
                 gen_ctx.builder.func.dfg.insts[last_inst].opcode().is_terminator()
            } else { false };
            
            if !is_terminated {
                 let zero = gen_ctx.builder.ins().iconst(types::I32, 0);
                 gen_ctx.builder.ins().return_(&[zero]);
            }
        }
        
        builder.finalize();
        module.define_function(func_id, ctx)
            .map_err(|e| anyhow!("Failed to define main: {}", e))?;
        module.clear_context(ctx);
        
        Ok(())
    }

    fn generate_function(
        &mut self,
        func_def: &FunctionDecl,
        module: &mut ObjectModule,
        ctx: &mut codegen::Context,
        printf_id: FuncId,
        gul_print_float_id: FuncId,
        gul_input_str_id: FuncId,
        gul_input_int_id: FuncId,
        gul_input_flt_id: FuncId,
        gul_table_alloc_id: FuncId,
        gul_table_set_col_name_id: FuncId,
        gul_table_set_row_id: FuncId,
        gul_malloc_id: FuncId,
        gul_list_alloc_id: FuncId,
        gul_list_push_id: FuncId,
        gul_dict_alloc_id: FuncId,
        gul_dict_set_id: FuncId,
        gul_set_alloc_id: FuncId,
        gul_set_add_id: FuncId,
        gul_list_get_id: FuncId,
        gul_list_len_id: FuncId,
        gul_dict_get_id: FuncId,
        gul_int_to_string_id: FuncId,
        gul_float_to_string_id: FuncId,
        gul_string_concat_id: FuncId,
        format_int_id: DataId,
        format_str_id: DataId,
        string_literals: &mut HashMap<String, DataId>,
        builtins: HashMap<String, FuncId>,
        functions: &HashMap<String, FuncId>,
    ) -> Result<()> {
        ctx.clear();
        let func_id = *functions.get(&func_def.name).ok_or(anyhow!("Function ID not found"))?;
        
        // Setup signature from Definition
        let mut sig = module.make_signature();
        
        for param in &func_def.parameters {
             let ty = if !param.type_annotation.is_empty() {
                    let t = &param.type_annotation;
                    if t == "int" { types::I64 }
                    else if t == "float" { types::F64 }
                    else if t == "str" { types::I64 } 
                    else { types::I64 }
                } else { types::I64 };
             sig.params.push(AbiParam::new(ty));
        }
        
        if !func_def.return_type.is_empty() {
             let ret_type = &func_def.return_type;
             let ty = if ret_type == "int" { types::I64 }
                else if ret_type == "float" { types::F64 }
                else if ret_type == "str" { types::I64 }
                else { types::I64 };
             sig.returns.push(AbiParam::new(ty));
        } else {
             sig.returns.push(AbiParam::new(types::I64));
        }
        
        ctx.func.signature = sig;

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        
        builder.switch_to_block(entry_block);
        // Workaround: Manually append block if switch_to_block failed to do so (bug in builder interaction?)
        if builder.func.layout.blocks().count() == 0 {
             builder.func.layout.append_block(entry_block);
        }
        builder.seal_block(entry_block);
        
        let mut variables = HashMap::new();
        let mut struct_layouts = HashMap::new();
        let mut var_counter = 0;
        
        {
            let mut gen_ctx = GenContext {
                builder: &mut builder,
                module,
                variables: &mut variables,
                var_counter: &mut var_counter,
                printf_id,
                gul_print_float_id,
                gul_input_str_id,
                gul_input_int_id,
                gul_input_flt_id,
                gul_table_alloc_id,
                gul_table_set_col_name_id,
                gul_table_set_row_id,
                gul_malloc_id,
                gul_list_alloc_id,
                gul_list_push_id,
                gul_dict_alloc_id,
                gul_dict_set_id,
                gul_set_alloc_id,
                gul_set_add_id,
                gul_list_get_id,
                gul_list_len_id,
                gul_dict_get_id,
                gul_int_to_string_id,
                gul_float_to_string_id,
                gul_string_concat_id,
                format_int_id,
                format_str_id,
                string_literals,
                data_description: &mut self.data_description,
                builtins,
                functions,
                struct_layouts: &mut struct_layouts,
                current_func_name: func_def.name.clone(),
            };
            
            // Register params as variables
            for (i, param) in func_def.parameters.iter().enumerate() {
                let val = gen_ctx.builder.block_params(entry_block)[i];
                let var = Variable::new(*gen_ctx.var_counter); 
                *gen_ctx.var_counter += 1;
                gen_ctx.builder.declare_var(var, gen_ctx.builder.func.dfg.value_type(val));
                gen_ctx.builder.def_var(var, val);
                
                // Determine param type
                let type_name = if !param.type_annotation.is_empty() { param.type_annotation.clone() } else { "int".to_string() };
                gen_ctx.variables.insert(param.name.clone(), (var, type_name));
            }
            
            // Generate statements
            for stmt in &func_def.body {
                if Self::generate_statement(stmt, &mut gen_ctx)? { break; }
            }
            
            // Make sure block is terminated
            let current_block = gen_ctx.builder.current_block().unwrap();
            let is_terminated = if let Some(last_inst) = gen_ctx.builder.func.layout.last_inst(current_block) {
                 gen_ctx.builder.func.dfg.insts[last_inst].opcode().is_terminator()
            } else { false };
            
            if !is_terminated {
                 // Add default ret 0
                 let zero = gen_ctx.builder.ins().iconst(types::I64, 0);
                 gen_ctx.builder.ins().return_(&[zero]);
            }
        }
        
        builder.finalize();
        module.define_function(func_id, ctx)
            .map_err(|e| anyhow!("Failed to define function {}: {}", func_def.name, e))?;
        module.clear_context(ctx);
        Ok(())
    }



    fn infer_expression_type(expr: &Expression, ctx: &GenContext) -> String {
        match expr {
            Expression::Literal(lit) => match lit.value_type {
                TokenType::Integer => "int".to_string(),
                TokenType::Float => "float".to_string(),
                TokenType::String => "str".to_string(),
                TokenType::TrueKeyword | TokenType::FalseKeyword => "bool".to_string(),
                _ => "unknown".to_string(),
            },
            Expression::Identifier(ident) => {
                if let Some((_, type_name)) = ctx.variables.get(&ident.name) {
                    type_name.clone()
                } else {
                    "unknown".to_string()
                }
            },
            Expression::BinaryOp(binop) => {
                let left_t = Self::infer_expression_type(&binop.left, ctx);
                let right_t = Self::infer_expression_type(&binop.right, ctx);
                
                if binop.operator == TokenType::Plus && (left_t == "str" || right_t == "str") {
                    return "str".to_string();
                }
                
                if left_t == "float" || right_t == "float" {
                    return "float".to_string();
                }
                
                if left_t == "int" && right_t == "int" {
                    return "int".to_string();
                }

                left_t
            },
            Expression::TypeConstructor(tc) => {
                let name = tc.type_name.trim_start_matches('@');
                if name == "str" { "str".to_string() }
                else if name == "int" { "int".to_string() }
                else if name == "float" { "float".to_string() }
                else if name == "bool" { "bool".to_string() }
                else { name.to_string() }
            },
            Expression::Call(call) => {
                // Check if it's a known function with a return type
                if let Expression::Identifier(ident) = &*call.callee {
                    let name = ident.name.trim_start_matches('@');
                    if name == "input" || name == "input_str" { return "str".to_string(); }
                    if name == "input_int" { return "int".to_string(); }
                    if name == "input_flt" { return "float".to_string(); }
                    if name == "str" || name == "string" { return "str".to_string(); }
                    if name == "int" || name == "integer" { return "int".to_string(); }
                    if name == "float" || name == "flt" { return "float".to_string(); }
                    if name == "len" { return "int".to_string(); }
                }
                "unknown".to_string()
            }
            Expression::List(_) => "list".to_string(),
            Expression::Dict(_) => "dict".to_string(),
            Expression::Set(_) => "set".to_string(),
            Expression::Table(_) => "table".to_string(),
            Expression::DataFrame(_) => "dataframe".to_string(),
            _ => "unknown".to_string(),
        }
    }

    fn generate_statement(stmt: &Statement, ctx: &mut GenContext) -> Result<bool> {
        match stmt {
            Statement::LetDecl(let_stmt) => {
                let mut val = Self::generate_expression(&let_stmt.value, ctx)?;
                
                // Auto conversion
                if !let_stmt.type_annotation.is_empty() {
                     let expected = &let_stmt.type_annotation;
                     let actual = Self::infer_expression_type(&let_stmt.value, ctx);
                     
                     if expected == "str" {
                         if actual == "int" {
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_int_to_string_id, ctx.builder.func);
                             let call_res = ctx.builder.ins().call(func_ref, &[val]);
                             val = ctx.builder.inst_results(call_res)[0];
                         } else if actual == "float" {
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_float_to_string_id, ctx.builder.func);
                             // Need value to be F64 for float_to_string, but val might be I64 if it's unknown?
                             // infer says "float". Literal float gives F64.
                             let call_res = ctx.builder.ins().call(func_ref, &[val]);
                             val = ctx.builder.inst_results(call_res)[0];
                         }
                     }
                     if expected == "bool" && actual == "str" {
                         let mut sig = ctx.module.make_signature();
                         sig.params.push(AbiParam::new(types::I64));
                         sig.returns.push(AbiParam::new(types::I64)); 
                         let fid = ctx.module.declare_function("gul_str_to_bool", Linkage::Import, &sig)?;
                         let func_ref = ctx.module.declare_func_in_func(fid, ctx.builder.func);
                         let call_res = ctx.builder.ins().call(func_ref, &[val]);
                         val = ctx.builder.inst_results(call_res)[0];
                     }
                }
                
                let var_type = ctx.builder.func.dfg.value_type(val); // Infer type from value
                let var = Variable::new(*ctx.var_counter);
                *ctx.var_counter += 1;
                ctx.builder.declare_var(var, var_type);
                ctx.builder.def_var(var, val);
                let val_type_inferred = Self::infer_expression_type(&let_stmt.value, ctx);
                let type_name = if !let_stmt.type_annotation.is_empty() { 
                    let_stmt.type_annotation.clone() 
                } else if val_type_inferred != "unknown" { 
                    val_type_inferred 
                } else { 
                    "int".to_string() 
                };
                ctx.variables.insert(let_stmt.name.clone(), (var, type_name));
                Ok(false)
            }
            Statement::VarDecl(var_stmt) => {
                let mut val = Self::generate_expression(&var_stmt.value, ctx)?;

                // Auto conversion
                if !var_stmt.type_annotation.is_empty() {
                     let expected = &var_stmt.type_annotation;
                     let actual = Self::infer_expression_type(&var_stmt.value, ctx);
                     
                     if expected == "str" {
                         if actual == "int" {
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_int_to_string_id, ctx.builder.func);
                             let call_res = ctx.builder.ins().call(func_ref, &[val]);
                             val = ctx.builder.inst_results(call_res)[0];
                         } else if actual == "float" {
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_float_to_string_id, ctx.builder.func);
                             let call_res = ctx.builder.ins().call(func_ref, &[val]);
                             val = ctx.builder.inst_results(call_res)[0];
                         }
                     }
                     if expected == "bool" && actual == "str" {
                         let mut sig = ctx.module.make_signature();
                         sig.params.push(AbiParam::new(types::I64));
                         sig.returns.push(AbiParam::new(types::I64));
                         let fid = ctx.module.declare_function("gul_str_to_bool", Linkage::Import, &sig)?;
                         let func_ref = ctx.module.declare_func_in_func(fid, ctx.builder.func);
                         let call_res = ctx.builder.ins().call(func_ref, &[val]);
                         val = ctx.builder.inst_results(call_res)[0];
                     }
                }

                let var_type = ctx.builder.func.dfg.value_type(val); // Infer type from value
                let var = Variable::new(*ctx.var_counter);
                *ctx.var_counter += 1;
                ctx.builder.declare_var(var, var_type);
                ctx.builder.def_var(var, val);
                let val_type_inferred = Self::infer_expression_type(&var_stmt.value, ctx);
                let type_name = if !var_stmt.type_annotation.is_empty() { 
                    var_stmt.type_annotation.clone() 
                } else if val_type_inferred != "unknown" { 
                    val_type_inferred 
                } else { 
                    "int".to_string() 
                };
                ctx.variables.insert(var_stmt.name.clone(), (var, type_name));
                Ok(false)
            }
            Statement::AssignmentStmt(assign) => {
                let val = Self::generate_expression(&assign.value, ctx)?;
                match &assign.target {
                    Expression::Identifier(ident) => {
                        if let Some((var, _)) = ctx.variables.get(&ident.name) {
                            ctx.builder.def_var(*var, val);
                        }
                    }
                    Expression::Attribute(attr) => {
                         // Field assignment: obj.field = val
                         let obj_val = Self::generate_expression(&attr.object, ctx)?;
                         let obj_type = Self::infer_expression_type(&attr.object, ctx);
                         
                         let mut offset = 0;
                         let mut found = false;
                         
                         if let Some(layout) = ctx.struct_layouts.get(&obj_type) {
                             if let Some(off) = layout.get(&attr.attribute) {
                                  offset = *off;
                                  found = true;
                             }
                         }
                         
                         // Fallback to old hack if type not found
                         if !found {
                             for layout in ctx.struct_layouts.values() {
                                 if let Some(off) = layout.get(&attr.attribute) {
                                     offset = *off;
                                     found = true;
                                     break;
                                 }
                             }
                         }
                         if found {
                             let flags = MemFlags::new();
                             ctx.builder.ins().store(flags, val, obj_val, offset as i32);
                         }
                    }
                    _ => {}
                }
                Ok(false)
            }
            Statement::ExpressionStmt(expr_stmt) => {
                Self::generate_expression(&expr_stmt.expression, ctx)?;
                Ok(false)
            }
            Statement::WhileStmt(while_stmt) => {
                let header = ctx.builder.create_block();
                let body_block = ctx.builder.create_block();
                let exit = ctx.builder.create_block();
                
                ctx.builder.ins().jump(header, &[]);
                ctx.builder.switch_to_block(header);
                
                // Ensure all variables are visible in header for phi node construction
                let var_keys: Vec<String> = ctx.variables.keys().cloned().collect();
                for k in var_keys {
                    if let Some((var, _)) = ctx.variables.get(&k) {
                        ctx.builder.use_var(*var);
                    }
                }
                
                let cond_val = Self::generate_expression(&while_stmt.condition, ctx)?;
                ctx.builder.ins().brif(cond_val, body_block, &[], exit, &[]);
                
                ctx.builder.switch_to_block(body_block);
                let mut terminated = false;
                for s in &while_stmt.body {
                    if Self::generate_statement(s, ctx)? {
                        terminated = true;
                        break;
                    }
                }
                if !terminated {
                    ctx.builder.ins().jump(header, &[]);
                }
                
                ctx.builder.switch_to_block(exit);
                ctx.builder.seal_block(header);
                ctx.builder.seal_block(body_block);
                ctx.builder.seal_block(exit);
                Ok(false)
            }
            Statement::ForStmt(for_stmt) => {
                // Check if it's a range loop: for i in start..end
                if let Expression::BinaryOp(binop) = &for_stmt.iterable {
                    if binop.operator == TokenType::DotDot {
                        let start_val = Self::generate_expression(&binop.left, ctx)?;
                        let end_val = Self::generate_expression(&binop.right, ctx)?;
                        
                        let header = ctx.builder.create_block();
                        let body_block = ctx.builder.create_block();
                        let exit = ctx.builder.create_block();
                        
                        let index_var_id = *ctx.var_counter;
                        *ctx.var_counter += 1;
                        let index_var = Variable::new(index_var_id);
                        ctx.builder.declare_var(index_var, types::I64);
                        ctx.builder.def_var(index_var, start_val);
                        
                        ctx.builder.ins().jump(header, &[]);
                        ctx.builder.switch_to_block(header);
                        
                        // Ensure all variables are visible in header for phi node construction
                        let var_keys: Vec<String> = ctx.variables.keys().cloned().collect();
                        for k in var_keys {
                            if let Some((var, _)) = ctx.variables.get(&k) {
                                ctx.builder.use_var(*var);
                            }
                        }
                        
                        let current_index = ctx.builder.use_var(index_var);
                        let cond = ctx.builder.ins().icmp(IntCC::SignedLessThan, current_index, end_val);
                        ctx.builder.ins().brif(cond, body_block, &[], exit, &[]);
                        
                        ctx.builder.switch_to_block(body_block);
                        
                        let old_var = ctx.variables.get(&for_stmt.variable).cloned();
                        ctx.variables.insert(for_stmt.variable.clone(), (index_var, "int".to_string()));
                        
                        let mut terminated = false;
                        for s in &for_stmt.body {
                            if Self::generate_statement(s, ctx)? {
                                terminated = true;
                                break;
                            }
                        }
                        
                        if !terminated {
                            let one = ctx.builder.ins().iconst(types::I64, 1);
                            let next_index = ctx.builder.ins().iadd(current_index, one);
                            ctx.builder.def_var(index_var, next_index);
                            ctx.builder.ins().jump(header, &[]);
                        }
                        
                        ctx.builder.switch_to_block(exit);
                        if let Some(old) = old_var {
                            ctx.variables.insert(for_stmt.variable.clone(), old);
                        } else {
                            ctx.variables.remove(&for_stmt.variable);
                        }

                        ctx.builder.seal_block(header);
                        ctx.builder.seal_block(body_block);
                        ctx.builder.seal_block(exit);
                        return Ok(false);
                    }
                }

                // 1. Evaluate iterable (assume list if not range)
                let iterable_val = Self::generate_expression(&for_stmt.iterable, ctx)?;
                
                // 2. Get length of iterable (supporting list for now)
                let len_func_ref = ctx.module.declare_func_in_func(ctx.gul_list_len_id, ctx.builder.func);
                let call_res = ctx.builder.ins().call(len_func_ref, &[iterable_val]);
                let len_val = ctx.builder.inst_results(call_res)[0];
                
                // 3. Setup loop
                let header = ctx.builder.create_block();
                let body_block = ctx.builder.create_block();
                let exit = ctx.builder.create_block();
                
                // Index variable
                let index_var_id = *ctx.var_counter;
                *ctx.var_counter += 1;
                let index_var = Variable::new(index_var_id);
                ctx.builder.declare_var(index_var, types::I64);
                let zero = ctx.builder.ins().iconst(types::I64, 0);
                ctx.builder.def_var(index_var, zero);
                
                ctx.builder.ins().jump(header, &[]);
                ctx.builder.switch_to_block(header);
                
                // Ensure all variables are visible in header for phi node construction
                let var_keys: Vec<String> = ctx.variables.keys().cloned().collect();
                for k in var_keys {
                   if let Some((var, _)) = ctx.variables.get(&k) {
                       ctx.builder.use_var(*var);
                   }
                }
                
                // 4. Loop condition: index < length
                let current_index = ctx.builder.use_var(index_var);
                let cond = ctx.builder.ins().icmp(IntCC::SignedLessThan, current_index, len_val);
                ctx.builder.ins().brif(cond, body_block, &[], exit, &[]);
                
                ctx.builder.switch_to_block(body_block);
                
                // 5. Load current element: var = gul_list_get(iterable, index)
                let get_func_ref = ctx.module.declare_func_in_func(ctx.gul_list_get_id, ctx.builder.func);
                let get_res = ctx.builder.ins().call(get_func_ref, &[iterable_val, current_index]);
                let elem_val = ctx.builder.inst_results(get_res)[0];
                
                // Declare/define loop variable
                let loop_var_id = *ctx.var_counter;
                *ctx.var_counter += 1;
                let loop_var = Variable::new(loop_var_id);
                ctx.builder.declare_var(loop_var, types::I64);
                ctx.builder.def_var(loop_var, elem_val);
                
                // Save old variable to restore after loop (simple scoping)
                let old_var = ctx.variables.get(&for_stmt.variable).cloned();
                ctx.variables.insert(for_stmt.variable.clone(), (loop_var, "int".to_string()));
                
                // 6. Execute body
                let mut terminated = false;
                for s in &for_stmt.body {
                    if Self::generate_statement(s, ctx)? {
                        terminated = true;
                        break;
                    }
                }
                
                // 7. Increment index
                if !terminated {
                    let one = ctx.builder.ins().iconst(types::I64, 1);
                    let next_index = ctx.builder.ins().iadd(current_index, one);
                    ctx.builder.def_var(index_var, next_index);
                    ctx.builder.ins().jump(header, &[]);
                }
                
                ctx.builder.switch_to_block(exit);
                // Restore old variable
                if let Some(old) = old_var {
                    ctx.variables.insert(for_stmt.variable.clone(), old);
                } else {
                    ctx.variables.remove(&for_stmt.variable);
                }
                
                ctx.builder.seal_block(header);
                ctx.builder.seal_block(body_block);
                ctx.builder.seal_block(exit);
                Ok(false)
            }
            Statement::IfStmt(if_stmt) => {
                let then_block = ctx.builder.create_block();
                let merge_block = ctx.builder.create_block();
                
                // Track the current "else" or "next condition" block
                let mut current_false_block = ctx.builder.create_block();
                
                let cond_val = Self::generate_expression(&if_stmt.condition, ctx)?;
                ctx.builder.ins().brif(cond_val, then_block, &[], current_false_block, &[]);
                
                // Then block
                ctx.builder.switch_to_block(then_block);
                ctx.builder.seal_block(then_block);
                let mut then_terminated = false;
                for s in &if_stmt.then_body {
                    if Self::generate_statement(s, ctx)? {
                        then_terminated = true;
                        break;
                    }
                }
                if !then_terminated {
                    ctx.builder.ins().jump(merge_block, &[]);
                }
                
                // Elif blocks
                for elif in &if_stmt.elif_clauses {
                    let next_true_block = ctx.builder.create_block();
                    let next_false_block = ctx.builder.create_block();
                    
                    ctx.builder.switch_to_block(current_false_block);
                    ctx.builder.seal_block(current_false_block);
                    
                    let elif_cond = Self::generate_expression(&elif.condition, ctx)?;
                    ctx.builder.ins().brif(elif_cond, next_true_block, &[], next_false_block, &[]);
                    
                    ctx.builder.switch_to_block(next_true_block);
                    ctx.builder.seal_block(next_true_block);
                    let mut elif_terminated = false;
                    for s in &elif.body {
                        if Self::generate_statement(s, ctx)? {
                            elif_terminated = true;
                            break;
                        }
                    }
                    if !elif_terminated {
                        ctx.builder.ins().jump(merge_block, &[]);
                    }
                    
                    current_false_block = next_false_block;
                }
                
                // Else block
                ctx.builder.switch_to_block(current_false_block);
                ctx.builder.seal_block(current_false_block);
                let mut else_terminated = false;
                for s in &if_stmt.else_body {
                    if Self::generate_statement(s, ctx)? {
                        else_terminated = true;
                        break;
                    }
                }
                if !else_terminated {
                    ctx.builder.ins().jump(merge_block, &[]);
                }
                
                ctx.builder.switch_to_block(merge_block);
                ctx.builder.seal_block(merge_block);
                Ok(false)
            }
            Statement::StructDecl(stmt) => {
                let mut layout = HashMap::new();
                for (i, field) in stmt.fields.iter().enumerate() {
                    layout.insert(field.name.clone(), i * 8); 
                }
                ctx.struct_layouts.insert(stmt.name.clone(), layout);
                Ok(false)
            }
            Statement::ForeignCodeBlock(stmt) => {
                 let func_id_opt = ctx.builtins.get("gul_exec_foreign").cloned();
                 if let Some(func_id) = func_id_opt {
                     let lang_val = Self::generate_string_literal(&stmt.language, ctx)?;
                     let code_val = Self::generate_string_literal(&stmt.code, ctx)?;
                     let func_ref = ctx.module.declare_func_in_func(func_id, ctx.builder.func);
                     ctx.builder.ins().call(func_ref, &[lang_val, code_val]);
                 }
                 Ok(false)
            }
            Statement::ReturnStmt(ret) => {
                 if let Some(val_expr) = &ret.value {
                     let val = Self::generate_expression(val_expr, ctx)?;
                     ctx.builder.ins().return_(&[val]);
                 } else {
                     ctx.builder.ins().return_(&[]);
                 }
                 Ok(true)
            }
            Statement::ImportStmt(_) => {
                Ok(false)
            }
            _ => Ok(false),
        }
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
                if let Some((var, _)) = ctx.variables.get(&ident.name) {
                    Ok(ctx.builder.use_var(*var))
                } else {
                    Ok(ctx.builder.ins().iconst(types::I64, 0))
                }
            }
            Expression::BinaryOp(binop) => {
                let mut left = Self::generate_expression(&binop.left, ctx)?;
                let mut right = Self::generate_expression(&binop.right, ctx)?;
                
                use crate::lexer::token::TokenType;
                
                let left_type = ctx.builder.func.dfg.value_type(left);
                let right_type = ctx.builder.func.dfg.value_type(right);
                
                // Type Promotion: Int to Float
                if left_type == types::I64 && right_type == types::F64 {
                    left = ctx.builder.ins().fcvt_from_sint(types::F64, left);
                } else if left_type == types::F64 && right_type == types::I64 {
                    right = ctx.builder.ins().fcvt_from_sint(types::F64, right);
                }

                let is_float = ctx.builder.func.dfg.value_type(left) == types::F64;
                
                // Handle String Concatenation
                if binop.operator == TokenType::Plus && !is_float {
                    let left_t = Self::infer_expression_type(&binop.left, ctx);
                    let right_t = Self::infer_expression_type(&binop.right, ctx);
                    if left_t == "str" || right_t == "str" {
                         // Promote both to string if needed? GUL usually expects both to be str or handles via auto-conv in assignment
                         // For now assume if one is str and operator is +, we use string_concat
                         let func_ref = ctx.module.declare_func_in_func(ctx.gul_string_concat_id, ctx.builder.func);
                         let call_res = ctx.builder.ins().call(func_ref, &[left, right]);
                         return Ok(ctx.builder.inst_results(call_res)[0]);
                    }
                }

                match binop.operator {
                    TokenType::Plus => {
                         if is_float { Ok(ctx.builder.ins().fadd(left, right)) }
                         else { Ok(ctx.builder.ins().iadd(left, right)) }
                    },
                    TokenType::Minus => {
                         if is_float { Ok(ctx.builder.ins().fsub(left, right)) }
                         else { Ok(ctx.builder.ins().isub(left, right)) }
                    },
                    TokenType::Star => {
                         if is_float { Ok(ctx.builder.ins().fmul(left, right)) }
                         else { Ok(ctx.builder.ins().imul(left, right)) }
                    },
                    TokenType::Slash => {
                         if is_float { Ok(ctx.builder.ins().fdiv(left, right)) }
                         else { Ok(ctx.builder.ins().sdiv(left, right)) }
                    },
                    TokenType::Greater => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::GreaterThan, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::SignedGreaterThan, left, right)) }
                    },
                    TokenType::Less => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::LessThan, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::SignedLessThan, left, right)) }
                    },
                    TokenType::GreaterEq => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, left, right)) }
                    },
                    TokenType::LessEq => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::LessThanOrEqual, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::SignedLessThanOrEqual, left, right)) }
                    },
                    TokenType::EqualEqual => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::Equal, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::Equal, left, right)) }
                    },
                    TokenType::NotEqual => {
                         if is_float { Ok(ctx.builder.ins().fcmp(FloatCC::NotEqual, left, right)) }
                         else { Ok(ctx.builder.ins().icmp(IntCC::NotEqual, left, right)) }
                    },
                    TokenType::And => {
                        Ok(ctx.builder.ins().band(left, right))
                    },
                    TokenType::Or => {
                        Ok(ctx.builder.ins().bor(left, right))
                    },
                    _ => Ok(ctx.builder.ins().iconst(types::I64, 0))
                }
            }
            Expression::Attribute(attr) => {
                let obj_val = Self::generate_expression(&attr.object, ctx)?;
                // Find offset by searching all layouts (Hack due to lack of type info)
                let mut offset = 0;
                let mut found = false;
                for layout in ctx.struct_layouts.values() {
                    if let Some(off) = layout.get(&attr.attribute) {
                        offset = *off;
                        found = true;
                        break;
                    }
                }
                if found {
                    let flags = MemFlags::new();
                    Ok(ctx.builder.ins().load(types::I64, flags, obj_val, offset as i32))
                } else if attr.attribute == "len" {
                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_list_len_id, ctx.builder.func);
                    let call_result = ctx.builder.ins().call(func_ref, &[obj_val]);
                    Ok(ctx.builder.inst_results(call_result)[0])
                } else {
                    Ok(ctx.builder.ins().iconst(types::I64, 0))
                }

            }
            Expression::Index(idx_expr) => {
                let obj_val = Self::generate_expression(&idx_expr.object, ctx)?;
                let idx_val = Self::generate_expression(&idx_expr.index, ctx)?;
                
                // Dispatch based on type
                let obj_type = Self::infer_expression_type(&idx_expr.object, ctx);
                
                if obj_type == "dict" {
                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_dict_get_id, ctx.builder.func);
                    let call_result = ctx.builder.ins().call(func_ref, &[obj_val, idx_val]);
                    Ok(ctx.builder.inst_results(call_result)[0])
                } else {
                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_list_get_id, ctx.builder.func);
                    let call_result = ctx.builder.ins().call(func_ref, &[obj_val, idx_val]);
                    Ok(ctx.builder.inst_results(call_result)[0])
                }

            }
            Expression::Call(call) => {
                if let Expression::Attribute(attr) = call.callee.as_ref() {
                    if attr.attribute == "push" || attr.attribute == "append" {
                         let list_val = Self::generate_expression(&attr.object, ctx)?;
                         if !call.arguments.is_empty() {
                             let item_val = Self::generate_expression(&call.arguments[0], ctx)?;
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_list_push_id, ctx.builder.func);
                             let call_res = ctx.builder.ins().call(func_ref, &[list_val, item_val]);
                             // Helper returns list usually, return it
                             if ctx.builder.inst_results(call_res).len() > 0 {
                                 return Ok(ctx.builder.inst_results(call_res)[0]);
                             } else {
                                 return Ok(list_val);
                             }
                         }
                    }
                }

                // Check for Struct Constructor
                if let Expression::Identifier(ident) = call.callee.as_ref() {
                    if let Some(layout) = ctx.struct_layouts.get(&ident.name) {
                        // Struct Constructor found!
                        let size = (layout.len() * 8) as i64;
                        let size_val = ctx.builder.ins().iconst(types::I64, size);
                        
                        if let Some(malloc_id) = ctx.builtins.get("gul_malloc") {
                            let malloc_ref = ctx.module.declare_func_in_func(*malloc_id, ctx.builder.func);
                            let call_result = ctx.builder.ins().call(malloc_ref, &[size_val]);
                            return Ok(ctx.builder.inst_results(call_result)[0]);
                        } else {
                            // Fallback if malloc not found? Should not happen if registered.
                            return Ok(ctx.builder.ins().iconst(types::I64, 0));
                        }
                    }

                    if (ident.name == "print" || ident.name == "println") && !call.arguments.is_empty() {
                        let arg = Self::generate_expression(&call.arguments[0], ctx)?;
                        let arg_type = ctx.builder.func.dfg.value_type(arg);
                        
                        if arg_type == types::F64 {
                             let func_ref = ctx.module.declare_func_in_func(ctx.gul_print_float_id, ctx.builder.func);
                             let call_result = ctx.builder.ins().call(func_ref, &[arg]);
                             return Ok(ctx.builder.inst_results(call_result)[0]);
                        } 
                        
                        // Use infer_expression_type to check if argument matches string
                        let arg_type = Self::infer_expression_type(&call.arguments[0], ctx);
                        let is_str = arg_type == "str"; 

                        if is_str {
                             let printf_ref = ctx.module.declare_func_in_func(ctx.printf_id, ctx.builder.func);
                             let fmt_id = ctx.format_str_id; 
                             let fmt_val = ctx.module.declare_data_in_func(fmt_id, ctx.builder.func);
                             let fmt_ptr = ctx.builder.ins().global_value(types::I64, fmt_val);
                             // Need to ensure arg is treated as pointer (I64), which it is by default for non-float
                             let call_result = ctx.builder.ins().call(printf_ref, &[fmt_ptr, arg]);
                             return Ok(ctx.builder.inst_results(call_result)[0]);
                        } else {
                             let printf_ref = ctx.module.declare_func_in_func(ctx.printf_id, ctx.builder.func);
                             let fmt_id = ctx.format_int_id; 
                             let fmt_val = ctx.module.declare_data_in_func(fmt_id, ctx.builder.func);
                             let fmt_ptr = ctx.builder.ins().global_value(types::I64, fmt_val);
                             let call_result = ctx.builder.ins().call(printf_ref, &[fmt_ptr, arg]);
                             return Ok(ctx.builder.inst_results(call_result)[0]);
                        }
                    }
                    
                    if ident.name == "len" && !call.arguments.is_empty() {
                        let arg_expr = &call.arguments[0];
                        let arg_val = Self::generate_expression(arg_expr, ctx)?;
                        let arg_type = Self::infer_expression_type(arg_expr, ctx);
                        
                        let fid = if arg_type == "dict" {
                            let mut sig = ctx.module.make_signature();
                            sig.params.push(AbiParam::new(types::I64));
                            sig.returns.push(AbiParam::new(types::I64));
                            ctx.module.declare_function("gul_dict_len", Linkage::Import, &sig).map_err(|e| anyhow!("{}", e))?
                        } else if arg_type == "str" {
                            let mut sig = ctx.module.make_signature();
                            sig.params.push(AbiParam::new(types::I64));
                            sig.returns.push(AbiParam::new(types::I64));
                            ctx.module.declare_function("gul_string_len", Linkage::Import, &sig).map_err(|e| anyhow!("{}", e))?
                        } else {
                            ctx.gul_list_len_id
                        };
                        
                        let func_ref = ctx.module.declare_func_in_func(fid, ctx.builder.func);
                        let call_result = ctx.builder.ins().call(func_ref, &[arg_val]);
                        return Ok(ctx.builder.inst_results(call_result)[0]);
                    }
                    
                    if ident.name == "input" {
                         // ... (Input logic simplified for brevity in verification, assuming str)
                         let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_str_id, ctx.builder.func);
                         let call_result = ctx.builder.ins().call(func_ref, &[]);
                         return Ok(ctx.builder.inst_results(call_result)[0]);
                    }
                    
                    if let Some(fid) = ctx.functions.get(&ident.name) {
                        let func_ref = ctx.module.declare_func_in_func(*fid, ctx.builder.func);
                        let mut args = vec![];
                        for arg_expr in &call.arguments {
                             args.push(Self::generate_expression(arg_expr, ctx)?);
                        }
                        let call_result = ctx.builder.ins().call(func_ref, &args);
                        let results = ctx.builder.inst_results(call_result);
                        if !results.is_empty() {
                            return Ok(results[0]);
                        } else {
                            return Ok(ctx.builder.ins().iconst(types::I64, 0));
                        }
                    }

                    if let Some(fid) = ctx.builtins.get(&ident.name) {
                        let func_ref = ctx.module.declare_func_in_func(*fid, ctx.builder.func);
                        let mut args = vec![];
                        for arg_expr in &call.arguments {
                             args.push(Self::generate_expression(arg_expr, ctx)?);
                        }
                        let call_result = ctx.builder.ins().call(func_ref, &args);
                        let results = ctx.builder.inst_results(call_result);
                        if !results.is_empty() {
                            return Ok(results[0]);
                        } else {
                            return Ok(ctx.builder.ins().iconst(types::I64, 0));
                        }
                    }
                }
                // Handle @int(input()) etc...
                if let Expression::Identifier(ident) = call.callee.as_ref() { // Re-check ident for @-prefix
                    if ident.name.starts_with('@') && !call.arguments.is_empty() {
                        // Check if argument is input() call
                        if let Expression::Call(inner_call) = &call.arguments[0] {
                            if let Expression::Identifier(inner_ident) = inner_call.callee.as_ref() {
                                if inner_ident.name == "input" {
                                    // Route to typed input function based on @-prefix
                                    match ident.name.as_str() {
                                        "@int" => {
                                            let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_int_id, ctx.builder.func);
                                            let call_result = ctx.builder.ins().call(func_ref, &[]);
                                            return Ok(ctx.builder.inst_results(call_result)[0]);
                                        }
                                        "@flt" | "@float" => {
                                            let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_flt_id, ctx.builder.func);
                                            let call_result = ctx.builder.ins().call(func_ref, &[]);
                                            return Ok(ctx.builder.inst_results(call_result)[0]);
                                        }
                                        "@str" | "@string" => {
                                            let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_str_id, ctx.builder.func);
                                            let call_result = ctx.builder.ins().call(func_ref, &[]);
                                            return Ok(ctx.builder.inst_results(call_result)[0]);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        
                        // Handle type casting for non-input arguments (e.g., @flt(42), @flt(x))
                        let inner_val = Self::generate_expression(&call.arguments[0], ctx)?;
                        let inner_type = ctx.builder.func.dfg.value_type(inner_val);
                        
                        match ident.name.as_str() {
                            "@flt" | "@float" => {
                                // Cast to f64
                                if inner_type == types::F64 {
                                    return Ok(inner_val); // Already float
                                } else {
                                    return Ok(ctx.builder.ins().fcvt_from_sint(types::F64, inner_val));
                                }
                            }
                            "@int" => {
                                // Cast to i64
                                if inner_type == types::I64 {
                                    return Ok(inner_val); // Already int
                                } else if inner_type == types::F64 {
                                    return Ok(ctx.builder.ins().fcvt_to_sint(types::I64, inner_val));
                                } else {
                                    return Ok(inner_val); // Unknown, return as-is
                                }
                            }
                            "@str" | "@string" => {
                                // Call gul_int_to_string or gul_float_to_string based on arg type
                                let arg_type = ctx.builder.func.dfg.value_type(inner_val);
                                if arg_type == types::F64 {
                                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_float_to_string_id, ctx.builder.func);
                                    let call_result = ctx.builder.ins().call(func_ref, &[inner_val]);
                                    return Ok(ctx.builder.inst_results(call_result)[0]);
                                } else {
                                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_int_to_string_id, ctx.builder.func);
                                    let call_result = ctx.builder.ins().call(func_ref, &[inner_val]);
                                    return Ok(ctx.builder.inst_results(call_result)[0]);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(ctx.builder.ins().iconst(types::I64, 0))
            }
            // Handle type constructor: @int(input()), @flt(input()), @str(input())
            Expression::TypeConstructor(tc) => {
                // Check if argument is an input() call
                if let Expression::Call(call) = tc.argument.as_ref() {
                    if let Expression::Identifier(ident) = call.callee.as_ref() {
                        if ident.name == "input" {
                            // Call the appropriate typed input function
                            match tc.type_name.as_str() {
                                "int" => {
                                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_int_id, ctx.builder.func);
                                    let call_result = ctx.builder.ins().call(func_ref, &[]);
                                    return Ok(ctx.builder.inst_results(call_result)[0]);
                                }
                                "flt" | "float" => {
                                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_flt_id, ctx.builder.func);
                                    let call_result = ctx.builder.ins().call(func_ref, &[]);
                                    return Ok(ctx.builder.inst_results(call_result)[0]);
                                }
                                "str" | "string" => {
                                    let func_ref = ctx.module.declare_func_in_func(ctx.gul_input_str_id, ctx.builder.func);
                                    let call_result = ctx.builder.ins().call(func_ref, &[]);
                                    return Ok(ctx.builder.inst_results(call_result)[0]);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                
                // Actual Casting Logic
                let inner_val = Self::generate_expression(&tc.argument, ctx)?;
                match tc.type_name.as_str() {
                    "@chan" => {
                        // @chan(capacity) -> gul_chan_create
                        let mut sig = ctx.module.make_signature();
                        sig.params.push(AbiParam::new(types::I64));
                        sig.returns.push(AbiParam::new(types::I64));
                        let func_id = ctx.module.declare_function("gul_chan_create", Linkage::Import, &sig)?;
                        let func_ref = ctx.module.declare_func_in_func(func_id, ctx.builder.func);
                        let call_result = ctx.builder.ins().call(func_ref, &[inner_val]);
                        Ok(ctx.builder.inst_results(call_result)[0])
                    }
                    "flt" | "float" => {
                         // Cast to f64 (assume from i64)
                         Ok(ctx.builder.ins().fcvt_from_sint(types::F64, inner_val))
                    }
                    "int" => {
                         // Cast to i64 (assume from f64)
                         Ok(ctx.builder.ins().fcvt_to_sint(types::I64, inner_val))
                    }
                     _ => Ok(inner_val) // No-op for others
                }
            }
            // Handle Table literal: @tabl { (col1, col2): row1: {1, 2} }
            Expression::Table(table) => {
                let col_count = table.columns.len() as i64;
                let row_count = table.rows.len() as i64;
                
                // Allocate table
                let col_count_val = ctx.builder.ins().iconst(types::I64, col_count);
                let row_count_val = ctx.builder.ins().iconst(types::I64, row_count);
                let alloc_ref = ctx.module.declare_func_in_func(ctx.gul_table_alloc_id, ctx.builder.func);
                let call_result = ctx.builder.ins().call(alloc_ref, &[col_count_val, row_count_val]);
                let table_ptr = ctx.builder.inst_results(call_result)[0];
                
                // Set column names
                let set_col_ref = ctx.module.declare_func_in_func(ctx.gul_table_set_col_name_id, ctx.builder.func);
                for (i, col_name) in table.columns.iter().enumerate() {
                    let idx_val = ctx.builder.ins().iconst(types::I64, i as i64);
                    let name_val = Self::generate_string_literal(col_name, ctx)?;
                    ctx.builder.ins().call(set_col_ref, &[table_ptr, idx_val, name_val]);
                }
                
                // Set rows
                let set_row_ref = ctx.module.declare_func_in_func(ctx.gul_table_set_row_id, ctx.builder.func);
                let malloc_ref = ctx.module.declare_func_in_func(ctx.gul_malloc_id, ctx.builder.func);
                
                for (i, row) in table.rows.iter().enumerate() {
                    let idx_val = ctx.builder.ins().iconst(types::I64, i as i64);
                    let name_val = Self::generate_string_literal(&row.name, ctx)?;
                    
                    // Allocate array for row values (8 bytes per double)
                    let row_size = ctx.builder.ins().iconst(types::I64, (row.values.len() * 8) as i64);
                    let malloc_call = ctx.builder.ins().call(malloc_ref, &[row_size]);
                    let values_ptr = ctx.builder.inst_results(malloc_call)[0];
                    
                    // Store each value
                    for (j, val_expr) in row.values.iter().enumerate() {
                        let val = Self::generate_expression(val_expr, ctx)?;
                        // Convert to f64 if needed
                        let val_type = ctx.builder.func.dfg.value_type(val);
                        let f64_val = if val_type == types::I64 {
                            ctx.builder.ins().fcvt_from_sint(types::F64, val)
                        } else {
                            val
                        };
                        let offset = (j * 8) as i32;
                        let flags = MemFlags::new();
                        ctx.builder.ins().store(flags, f64_val, values_ptr, offset);
                    }
                    
                    ctx.builder.ins().call(set_row_ref, &[table_ptr, idx_val, name_val, values_ptr]);
                }
                
                Ok(table_ptr)
            }
            // Handle Await expression
            Expression::Await(await_expr) => {
                // For now, just evaluate the inner expression
                Self::generate_expression(&await_expr.value, ctx)
            }
            // Handle List literal: @list[1, 2, 3]
            Expression::List(list) => {
                let len = list.elements.len() as i64;
                
                // Allocate list with initial capacity
                let capacity_val = ctx.builder.ins().iconst(types::I64, len.max(8));
                let alloc_ref = ctx.module.declare_func_in_func(ctx.gul_list_alloc_id, ctx.builder.func);
                let call_result = ctx.builder.ins().call(alloc_ref, &[capacity_val]);
                let list_ptr = ctx.builder.inst_results(call_result)[0];
                
                // Push each element
                let push_ref = ctx.module.declare_func_in_func(ctx.gul_list_push_id, ctx.builder.func);
                for elem_expr in &list.elements {
                    let elem_val = Self::generate_expression(elem_expr, ctx)?;
                    ctx.builder.ins().call(push_ref, &[list_ptr, elem_val]);
                }
                
                Ok(list_ptr)
            }
            // Handle Dict literal: @dict{key: value, ...}
            Expression::Dict(dict) => {
                let len = dict.pairs.len() as i64;
                
                // Allocate dict with initial capacity
                let capacity_val = ctx.builder.ins().iconst(types::I64, len.max(8));
                let alloc_ref = ctx.module.declare_func_in_func(ctx.gul_dict_alloc_id, ctx.builder.func);
                let call_result = ctx.builder.ins().call(alloc_ref, &[capacity_val]);
                let dict_ptr = ctx.builder.inst_results(call_result)[0];
                
                // Set each key-value pair
                let set_ref = ctx.module.declare_func_in_func(ctx.gul_dict_set_id, ctx.builder.func);
                for (key_expr, value_expr) in &dict.pairs {
                    let key_val = Self::generate_expression(key_expr, ctx)?;
                    let value_val = Self::generate_expression(value_expr, ctx)?;
                    ctx.builder.ins().call(set_ref, &[dict_ptr, key_val, value_val]);
                }
                
                Ok(dict_ptr)
            }
            // Handle Set literal: @set{1, 2, 3}
            Expression::Set(set) => {
                let len = set.elements.len() as i64;
                
                // Allocate set with initial capacity
                let capacity_val = ctx.builder.ins().iconst(types::I64, len.max(8));
                let alloc_ref = ctx.module.declare_func_in_func(ctx.gul_set_alloc_id, ctx.builder.func);
                let call_result = ctx.builder.ins().call(alloc_ref, &[capacity_val]);
                let set_ptr = ctx.builder.inst_results(call_result)[0];
                
                // Add each element
                let add_ref = ctx.module.declare_func_in_func(ctx.gul_set_add_id, ctx.builder.func);
                for elem_expr in &set.elements {
                    let elem_val = Self::generate_expression(elem_expr, ctx)?;
                    ctx.builder.ins().call(add_ref, &[set_ptr, elem_val]);
                }
                
                Ok(set_ptr)
            }
            // Handle DataFrame literal: @frame{...}
            Expression::DataFrame(df_expr) => {
                // Create DataFrame with columns
                let n_cols = df_expr.columns.len() as i64;
                let n_rows = 0; // Start with 0 rows
                
                let rows_val = ctx.builder.ins().iconst(types::I64, n_rows);
                let cols_val = ctx.builder.ins().iconst(types::I64, n_cols);
                
                // Declare and call gul_frame_create
                let mut sig = ctx.module.make_signature();
                sig.params.push(AbiParam::new(types::I64));
                sig.params.push(AbiParam::new(types::I64));
                sig.returns.push(AbiParam::new(types::I64));
                let func_id = ctx.module.declare_function("gul_frame_create", Linkage::Import, &sig)?;
                let func_ref = ctx.module.declare_func_in_func(func_id, ctx.builder.func);
                
                let call_result = ctx.builder.ins().call(func_ref, &[rows_val, cols_val]);
                let df_ptr = ctx.builder.inst_results(call_result)[0];
                
                // Set column names
                let mut set_col_sig = ctx.module.make_signature();
                set_col_sig.params.push(AbiParam::new(types::I64)); // df_ptr
                set_col_sig.params.push(AbiParam::new(types::I64)); // col_idx
                set_col_sig.params.push(AbiParam::new(types::I64)); // name_ptr
                let set_col_id = ctx.module.declare_function("gul_frame_set_column_name", Linkage::Import, &set_col_sig)?;
                let set_col_ref = ctx.module.declare_func_in_func(set_col_id, ctx.builder.func);
                
                for (i, col_name) in df_expr.columns.iter().enumerate() {
                    let idx_val = ctx.builder.ins().iconst(types::I64, i as i64);
                    let name_val = Self::generate_string_literal(col_name, ctx)?;
                    ctx.builder.ins().call(set_col_ref, &[df_ptr, idx_val, name_val]);
                }
                
                Ok(df_ptr)
            }
            _ => Ok(ctx.builder.ins().iconst(types::I64, 0))
        }
    }

}
