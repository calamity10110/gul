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
                name: "",
                imports: vec![],
                structs: vec![],
                impls: vec![],
                functions: vec![],
            },
            ,
            current_struct: "",
            pending_impls: HashMap::new(),
        };

    }
    pub fn build_module(&mut self, ast: Module)  ->  IRModule {
        self.current_module.name = ast.name;
        for stmt in ast.body {
            self.process_statement(stmt);
        }
        self.finalize_impls();
        return self.current_module;

    }
    pub fn process_statement(&mut self, stmt: Statement) {
        if stmt.stmt_type == StmtType::StructDecl {
            self.process_struct(stmt);
        }
        else if stmt.stmt_type == StmtType::FunctionDecl {
            self.process_function(stmt);
        }
        else if stmt.stmt_type == StmtType::ImportStmt {
            self.process_import(stmt);

        }
    }
    pub fn process_struct(&mut self, stmt: Statement) {
        let decl = stmt;
        let mut ir_struct = IRStruct {base: IRNode {node_type: IRNodeType::Struct, line: stmt.base.line, column: stmt.base.column}, name: decl.name, fields: vec![]};
        self.current_struct = decl.name;
        for member in decl.body {
            if member.stmt_type == StmtType::FieldDecl {
                let field = IRField {base: IRNode {node_type: IRNodeType::Field, line: member.base.line, column: member.base.column}, name: member.name, type_name: gul_type_to_rust(member.type_annotation), default_value: ""};
                ir_struct.fields.push(field);
            }
            else if member.stmt_type == StmtType::FunctionDecl {
                let ir_func = self.build_function(member);
                if !self.pending_impls.contains_key(decl.name) {
                    self.pending_impls[decl.name] = vec![];
                }
                self.pending_impls[decl.name].push(ir_func);
            }
        }
        self.current_module.structs.push(ir_struct);
        self.current_struct = "";

    }
    pub fn process_function(&mut self, stmt: Statement) {
        let ir_func = self.build_function(stmt);
        self.current_module.functions.push(ir_func);

    }
    pub fn build_function(&mut self, decl: FunctionDecl)  ->  IRFunction {
        let mut ir_func = IRFunction {base: IRNode {node_type: IRNodeType::Function, line: decl.base.line, column: decl.base.column}, name: decl.name, params: vec![], return_type: gul_type_to_rust(decl.return_type), body: vec![], is_method: false, receiver: ""};
        for param in decl.parameters {
            if param.name == "sel".to_string() || param.name == ".to_string()ref self".to_string() {
                ir_func.is_method = true;
                ir_func.receiver = param.name;
            }
            else {
                let ir_param = IRParameter {base: IRNode {node_type: IRNodeType::Parameter, line: param.base.line, column: param.base.column}, name: param.name, type_name: gul_type_to_rust(param.type_annotation), ownership: param.ownership_mode};
                ir_func.params.push(ir_param);
            }
        }
        return ir_func;

    }
    pub fn process_import(&mut self, stmt: Statement) {
        let parts = stmt.module_path;
        let mut path = "";
        for part in parts {
            if (path).len() > 0 {
                path = path + "::";
            }
            path = path + part;
        }
        self.current_module.imports.push(path);

    }
    pub fn finalize_impls(&mut self) {
        // pass

    }
}