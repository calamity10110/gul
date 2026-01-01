// WASM Backend Module for GUL
// Provides WebAssembly code generation, optimization, and browser integration

use std::collections::HashMap;

/// WASM code generator
#[derive(Debug)]
pub struct WasmCodeGen {
    functions: Vec<WasmFunction>,
    imports: Vec<WasmImport>,
    exports: Vec<WasmExport>,
    memory: WasmMemory,
}

#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub name: String,
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
    pub locals: Vec<WasmType>,
    pub body: Vec<WasmInstruction>,
}

#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub kind: WasmImportKind,
}

#[derive(Debug, Clone)]
pub enum WasmImportKind {
    Function {
        params: Vec<WasmType>,
        results: Vec<WasmType>,
    },
    Memory {
        min: u32,
        max: Option<u32>,
    },
    Table {
        element_type: WasmType,
        min: u32,
        max: Option<u32>,
    },
    Global {
        value_type: WasmType,
        mutable: bool,
    },
}

#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub kind: WasmExportKind,
}

#[derive(Debug, Clone)]
pub enum WasmExportKind {
    Function(usize),
    Memory(usize),
    Table(usize),
    Global(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

#[derive(Debug, Clone)]
pub enum WasmInstruction {
    // Control flow
    Block { result_type: Option<WasmType> },
    Loop { result_type: Option<WasmType> },
    If { result_type: Option<WasmType> },
    Else,
    End,
    Br(u32),
    BrIf(u32),
    Return,
    Call(u32),
    CallIndirect { type_idx: u32, table_idx: u32 },

    // Parametric
    Drop,
    Select,

    // Variable access
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Memory
    I32Load { align: u32, offset: u32 },
    I64Load { align: u32, offset: u32 },
    F32Load { align: u32, offset: u32 },
    F64Load { align: u32, offset: u32 },
    I32Store { align: u32, offset: u32 },
    I64Store { align: u32, offset: u32 },
    F32Store { align: u32, offset: u32 },
    F64Store { align: u32, offset: u32 },
    MemorySize,
    MemoryGrow,

    // Constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Numeric operations
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    // F32 operations
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
}

#[derive(Debug, Clone)]
pub struct WasmMemory {
    pub min_pages: u32,
    pub max_pages: Option<u32>,
    pub shared: bool,
}

impl WasmCodeGen {
    pub fn new() -> Self {
        WasmCodeGen {
            functions: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            memory: WasmMemory {
                min_pages: 1,
                max_pages: None,
                shared: false,
            },
        }
    }

    /// Add a function to the module
    pub fn add_function(&mut self, func: WasmFunction) {
        self.functions.push(func);
    }

    /// Add an import
    pub fn add_import(&mut self, import: WasmImport) {
        self.imports.push(import);
    }

    /// Add an export
    pub fn add_export(&mut self, export: WasmExport) {
        self.exports.push(export);
    }

    /// Set memory configuration
    pub fn set_memory(&mut self, min_pages: u32, max_pages: Option<u32>, shared: bool) {
        self.memory = WasmMemory {
            min_pages,
            max_pages,
            shared,
        };
    }

    /// Generate WASM binary
    pub fn generate(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // Magic number
        output.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
        // Version
        output.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

        // Type section
        self.encode_type_section(&mut output);

        // Import section
        if !self.imports.is_empty() {
            self.encode_import_section(&mut output);
        }

        // Function section
        if !self.functions.is_empty() {
            self.encode_function_section(&mut output);
        }

        // Memory section
        self.encode_memory_section(&mut output);

        // Export section
        if !self.exports.is_empty() {
            self.encode_export_section(&mut output);
        }

        // Code section
        if !self.functions.is_empty() {
            self.encode_code_section(&mut output);
        }

        output
    }

    fn encode_type_section(&self, output: &mut Vec<u8>) {
        // Section ID for type section
        output.push(0x01);

        let mut section_data = Vec::new();
        // Number of types
        self.encode_u32(self.functions.len() as u32, &mut section_data);

        for func in &self.functions {
            // Function type
            section_data.push(0x60);
            // Number of parameters
            self.encode_u32(func.params.len() as u32, &mut section_data);
            for param in &func.params {
                section_data.push(self.type_to_byte(param));
            }
            // Number of results
            self.encode_u32(func.results.len() as u32, &mut section_data);
            for result in &func.results {
                section_data.push(self.type_to_byte(result));
            }
        }

        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_import_section(&self, output: &mut Vec<u8>) {
        output.push(0x02); // Import section ID
        let mut section_data = Vec::new();
        self.encode_u32(self.imports.len() as u32, &mut section_data);
        // Encode imports (simplified)
        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_function_section(&self, output: &mut Vec<u8>) {
        output.push(0x03); // Function section ID
        let mut section_data = Vec::new();
        self.encode_u32(self.functions.len() as u32, &mut section_data);
        for i in 0..self.functions.len() {
            self.encode_u32(i as u32, &mut section_data);
        }
        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_memory_section(&self, output: &mut Vec<u8>) {
        output.push(0x05); // Memory section ID
        let mut section_data = Vec::new();
        self.encode_u32(1, &mut section_data); // One memory
        if let Some(max) = self.memory.max_pages {
            section_data.push(0x01); // Has maximum
            self.encode_u32(self.memory.min_pages, &mut section_data);
            self.encode_u32(max, &mut section_data);
        } else {
            section_data.push(0x00); // No maximum
            self.encode_u32(self.memory.min_pages, &mut section_data);
        }
        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_export_section(&self, output: &mut Vec<u8>) {
        output.push(0x07); // Export section ID
        let mut section_data = Vec::new();
        self.encode_u32(self.exports.len() as u32, &mut section_data);
        // Encode exports (simplified)
        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_code_section(&self, output: &mut Vec<u8>) {
        output.push(0x0A); // Code section ID
        let mut section_data = Vec::new();
        self.encode_u32(self.functions.len() as u32, &mut section_data);
        // Encode function bodies (simplified)
        self.encode_u32(section_data.len() as u32, output);
        output.extend_from_slice(&section_data);
    }

    fn encode_u32(&self, value: u32, output: &mut Vec<u8>) {
        // LEB128 encoding
        let mut val = value;
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            output.push(byte);
            if val == 0 {
                break;
            }
        }
    }

    fn type_to_byte(&self, wasm_type: &WasmType) -> u8 {
        match wasm_type {
            WasmType::I32 => 0x7F,
            WasmType::I64 => 0x7E,
            WasmType::F32 => 0x7D,
            WasmType::F64 => 0x7C,
            WasmType::V128 => 0x7B,
            WasmType::FuncRef => 0x70,
            WasmType::ExternRef => 0x6F,
        }
    }

    /// Optimize WASM code
    pub fn optimize(&mut self) {
        // Dead code elimination
        self.eliminate_dead_code();

        // Constant folding
        self.fold_constants();

        // Inline small functions
        self.inline_functions();
    }

    fn eliminate_dead_code(&mut self) {
        // Remove unreachable code after returns
        for func in &mut self.functions {
            let mut new_body = Vec::new();
            let mut after_return = false;

            for inst in &func.body {
                if after_return {
                    if matches!(inst, WasmInstruction::End) {
                        new_body.push(inst.clone());
                        after_return = false;
                    }
                    continue;
                }

                new_body.push(inst.clone());

                if matches!(inst, WasmInstruction::Return) {
                    after_return = true;
                }
            }

            func.body = new_body;
        }
    }

    fn fold_constants(&mut self) {
        // Constant folding optimization
        for func in &mut self.functions {
            let mut i = 0;
            while i + 2 < func.body.len() {
                match (&func.body[i], &func.body[i + 1], &func.body[i + 2]) {
                    (
                        WasmInstruction::I32Const(a),
                        WasmInstruction::I32Const(b),
                        WasmInstruction::I32Add,
                    ) => {
                        let result = a.wrapping_add(*b);
                        func.body.drain(i..i + 3);
                        func.body.insert(i, WasmInstruction::I32Const(result));
                    }
                    _ => i += 1,
                }
            }
        }
    }

    fn inline_functions(&mut self) {
        // Inline small functions (simplified)
        // In a real implementation, this would analyze call graphs
        // and inline functions below a certain size threshold
    }
}

impl Default for WasmCodeGen {
    fn default() -> Self {
        Self::new()
    }
}

/// wasm-bindgen integration
pub struct WasmBindgen {
    bindings: HashMap<String, BindingType>,
}

#[derive(Debug, Clone)]
pub enum BindingType {
    Function {
        name: String,
        params: Vec<String>,
        return_type: String,
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>,
    },
    Enum {
        name: String,
        variants: Vec<String>,
    },
}

impl WasmBindgen {
    pub fn new() -> Self {
        WasmBindgen {
            bindings: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, params: Vec<String>, return_type: String) {
        self.bindings.insert(
            name.clone(),
            BindingType::Function {
                name,
                params,
                return_type,
            },
        );
    }

    pub fn add_struct(&mut self, name: String, fields: Vec<(String, String)>) {
        self.bindings
            .insert(name.clone(), BindingType::Struct { name, fields });
    }

    pub fn generate_bindings(&self) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated wasm-bindgen bindings\n\n");

        for binding in self.bindings.values() {
            match binding {
                BindingType::Function {
                    name,
                    params,
                    return_type,
                } => {
                    output.push_str(&format!(
                        "#[wasm_bindgen]\npub fn {}({}) -> {} {{\n    todo!()\n}}\n\n",
                        name,
                        params.join(", "),
                        return_type
                    ));
                }
                BindingType::Struct { name, fields } => {
                    output.push_str(&format!("#[wasm_bindgen]\npub struct {} {{\n", name));
                    for (field_name, field_type) in fields {
                        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
                    }
                    output.push_str("}\n\n");
                }
                BindingType::Enum { name, variants } => {
                    output.push_str(&format!("#[wasm_bindgen]\npub enum {} {{\n", name));
                    for variant in variants {
                        output.push_str(&format!("    {},\n", variant));
                    }
                    output.push_str("}\n\n");
                }
            }
        }

        output
    }
}

impl Default for WasmBindgen {
    fn default() -> Self {
        Self::new()
    }
}

/// JavaScript interop for WASM
pub struct JsInterop {
    exports: Vec<String>,
    imports: Vec<String>,
}

impl JsInterop {
    pub fn new() -> Self {
        JsInterop {
            exports: Vec::new(),
            imports: Vec::new(),
        }
    }

    pub fn add_export(&mut self, name: String) {
        self.exports.push(name);
    }

    pub fn add_import(&mut self, name: String) {
        self.imports.push(name);
    }

    pub fn generate_js_glue(&self) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated JavaScript glue code\n\n");

        // Generate import statements
        if !self.imports.is_empty() {
            output.push_str("// Imports from JavaScript\n");
            for import in &self.imports {
                output.push_str(&format!("import {{ {} }} from './host.js';\n", import));
            }
            output.push('\n');
        }

        // Generate export wrappers
        if !self.exports.is_empty() {
            output.push_str("// Exports to JavaScript\n");
            output.push_str("export async function loadWasm() {\n");
            output.push_str("    const wasm = await import('./module.wasm');\n");
            output.push_str("    return {\n");
            for export in &self.exports {
                output.push_str(&format!("        {}: wasm.{},\n", export, export));
            }
            output.push_str("    };\n");
            output.push_str("}\n");
        }

        output
    }
}

impl Default for JsInterop {
    fn default() -> Self {
        Self::new()
    }
}

/// Browser API support
pub struct BrowserApi {
    apis: HashMap<String, Vec<String>>,
}

impl BrowserApi {
    pub fn new() -> Self {
        let mut api = BrowserApi {
            apis: HashMap::new(),
        };
        api.register_standard_apis();
        api
    }

    fn register_standard_apis(&mut self) {
        // DOM APIs
        self.apis.insert(
            "dom".to_string(),
            vec![
                "document".to_string(),
                "window".to_string(),
                "querySelector".to_string(),
                "createElement".to_string(),
                "appendChild".to_string(),
                "addEventListener".to_string(),
            ],
        );

        // Fetch API
        self.apis.insert(
            "fetch".to_string(),
            vec![
                "fetch".to_string(),
                "Request".to_string(),
                "Response".to_string(),
                "Headers".to_string(),
            ],
        );

        // Storage APIs
        self.apis.insert(
            "storage".to_string(),
            vec![
                "localStorage".to_string(),
                "sessionStorage".to_string(),
                "IndexedDB".to_string(),
            ],
        );

        // Canvas API
        self.apis.insert(
            "canvas".to_string(),
            vec![
                "getContext".to_string(),
                "fillRect".to_string(),
                "drawImage".to_string(),
                "clearRect".to_string(),
            ],
        );

        // WebGL API
        self.apis.insert(
            "webgl".to_string(),
            vec![
                "createShader".to_string(),
                "createProgram".to_string(),
                "drawArrays".to_string(),
                "uniform".to_string(),
            ],
        );
    }

    pub fn get_api(&self, name: &str) -> Option<&Vec<String>> {
        self.apis.get(name)
    }

    pub fn has_api(&self, name: &str) -> bool {
        self.apis.contains_key(name)
    }
}

impl Default for BrowserApi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_codegen_creation() {
        let codegen = WasmCodeGen::new();
        assert_eq!(codegen.functions.len(), 0);
    }

    #[test]
    fn test_add_function() {
        let mut codegen = WasmCodeGen::new();
        let func = WasmFunction {
            name: "add".to_string(),
            params: vec![WasmType::I32, WasmType::I32],
            results: vec![WasmType::I32],
            locals: vec![],
            body: vec![
                WasmInstruction::LocalGet(0),
                WasmInstruction::LocalGet(1),
                WasmInstruction::I32Add,
            ],
        };
        codegen.add_function(func);
        assert_eq!(codegen.functions.len(), 1);
    }

    #[test]
    fn test_wasm_generation() {
        let codegen = WasmCodeGen::new();
        let binary = codegen.generate();
        // Check magic number
        assert_eq!(&binary[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        // Check version
        assert_eq!(&binary[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_wasm_bindgen() {
        let mut bindgen = WasmBindgen::new();
        bindgen.add_function(
            "greet".to_string(),
            vec!["name: String".to_string()],
            "String".to_string(),
        );
        let bindings = bindgen.generate_bindings();
        assert!(bindings.contains("pub fn greet"));
    }

    #[test]
    fn test_js_interop() {
        let mut interop = JsInterop::new();
        interop.add_export("add".to_string());
        interop.add_import("console_log".to_string());
        let glue = interop.generate_js_glue();
        assert!(glue.contains("loadWasm"));
    }

    #[test]
    fn test_browser_api() {
        let api = BrowserApi::new();
        assert!(api.has_api("dom"));
        assert!(api.has_api("fetch"));
        assert!(api.has_api("storage"));
        assert!(api.has_api("canvas"));
        assert!(api.has_api("webgl"));
    }

    #[test]
    fn test_constant_folding() {
        let mut codegen = WasmCodeGen::new();
        let func = WasmFunction {
            name: "test".to_string(),
            params: vec![],
            results: vec![WasmType::I32],
            locals: vec![],
            body: vec![
                WasmInstruction::I32Const(5),
                WasmInstruction::I32Const(3),
                WasmInstruction::I32Add,
            ],
        };

        codegen.add_function(func.clone());
        codegen.optimize();

        // After optimization, should be folded to I32Const(8)
        assert_eq!(codegen.functions[0].body.len(), 1);
    }
}
