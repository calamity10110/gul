use crate::ir::ir_nodes::*;
use crate::ast::nodes::*;
use crate::builtins::codegen_helpers::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct IRBuilder {
    current_module: IRModule,
    current_struct: String,
    pub pending_impls: HashMap<String, Vec<IRFunction>>,

}
impl IRBuilder {
    pub fn create()  ->  IRBuilder {
        return IRBuilder {
            current_module: IRModule {
                base: IRNode {node_type: IRNodeType::Module, line: 0, column: 0},
                name: "".to_string(), // Fixed: String
                imports: vec![],
                structs: vec![],
                impls: vec![],
                functions: vec![],
            },
            current_struct: "".to_string(), // Fixed: String
            pending_impls: HashMap::new(),
        };

    }
    pub fn build_module(&mut self, ast: Program)  ->  IRModule { // Use Program
        // self.current_module.name = ast.name; // Program doesn't have name
        for stmt in ast.statements {
            self.process_statement(stmt);
        }
        self.finalize_impls();
        return self.current_module.clone();

    }
    pub fn process_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::StructDecl(decl) => self.process_struct(decl),
            Statement::FunctionDecl(decl) => self.process_function(decl),
            Statement::ImportStmt(decl) => self.process_import(decl),
            _ => {},
        }
    }
    pub fn process_struct(&mut self, decl: StructDecl) {
        let mut ir_struct = IRStruct {
            base: IRNode {node_type: IRNodeType::Struct, line: decl.node.line, column: decl.node.column},
            name: decl.name.clone(),
            fields: vec![]
        };
        self.current_struct = decl.name.clone();
        
        for field in decl.fields {
             let f = IRField {
                base: IRNode {node_type: IRNodeType::Field, line: decl.node.line, column: decl.node.column}, // Use struct node for now
                name: field.name,
                type_name: gul_type_to_rust(field.type_annotation),
                default_value: "".to_string()
             };
             ir_struct.fields.push(f);
        }
        
        self.current_module.structs.push(ir_struct);
        self.current_struct = "".to_string();

    }
    pub fn process_function(&mut self, decl: FunctionDecl) {
        let ir_func = self.build_function(decl);
        self.current_module.functions.push(ir_func);

    }
    pub fn build_function(&mut self, decl: FunctionDecl)  ->  IRFunction {
        let mut ir_func = IRFunction {
            base: IRNode {node_type: IRNodeType::Function, line: decl.node.line, column: decl.node.column},
            name: decl.name,
            params: vec![],
            return_type: gul_type_to_rust(decl.return_type),
            body: vec![],
            is_method: false,
            receiver: "".to_string()
        };
        for param in decl.parameters {
            if param.name == "self".to_string() || param.name == "&self".to_string() {
                ir_func.is_method = true;
                ir_func.receiver = param.name;
            }
            else {
                let ir_param = IRParameter {
                    base: IRNode {node_type: IRNodeType::Parameter, line: decl.node.line, column: decl.node.column}, // use func loc
                    name: param.name,
                    type_name: gul_type_to_rust(param.type_annotation),
                    ownership: param.ownership_mode
                };
                ir_func.params.push(ir_param);
            }
        }
        return ir_func;

    }
    pub fn process_import(&mut self, stmt: ImportStmt) {
        let parts = stmt.module_path;
        let mut path = "".to_string();
        for part in parts {
            if (path).len() > 0 {
                path = path + "::";
            }
            path = path + &part;
        }
        self.current_module.imports.push(path);

    }
    pub fn finalize_impls(&mut self) {
        // pass

    }
}