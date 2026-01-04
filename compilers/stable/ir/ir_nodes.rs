// GUL v3.2 Compiler - IR Node Definitions
// Intermediate Representation for code generation

// ================================================================================
// IR TYPES
// ================================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum IRNodeType {
    Module,
    Function,
    Struct,
    Impl,
    Field,
    Parameter,
    Statement,
    Expression,
    Type,

// Base IR node
}
#[derive(Debug, Clone, PartialEq)]
pub struct IRNode {
    pub node_type: IRNodeType,
    pub line: usize,
    pub column: usize,

// ================================================================================
// MODULE LEVEL
// ================================================================================

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRModule {
    pub base: IRNode,
    pub name: String,
    pub imports: Vec<String>,
    pub structs: Vec<IRStruct>,
    pub impls: Vec<IRImpl>,
    pub functions: Vec<IRFunction>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRStruct {
    pub base: IRNode,
    pub name: String,
    pub fields: Vec<IRField>,
    // No methods - methods go in IRImpl

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRImpl {
    pub base: IRNode,
    pub target_type: String,
    pub methods: Vec<IRFunction>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRField {
    pub base: IRNode,
    pub name: String,
    pub type_name: String,
    pub default_value: String, // Optional

// ================================================================================
// FUNCTIONS
// ================================================================================

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRFunction {
    pub base: IRNode,
    pub name: String,
    pub params: Vec<IRParameter>,
    pub return_type: String,
    pub body: Vec<IRStatement>,
    pub is_method: bool,
    pub receiver: String, // "self", "ref self", or empty

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRParameter {
    pub base: IRNode,
    pub name: String,
    pub type_name: String,
    pub ownership: String, // "borrow", "ref", "move", ""

// ================================================================================
// STATEMENTS
// ================================================================================

}
#[derive(Debug, Clone, PartialEq)]
pub enum IRStmtType {
    Let,
    Var,
    Assign,
    Return,
    If,
    While,
    For,
    Match,
    Expr,
    Block,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRStatement {
    pub base: IRNode,
    pub stmt_type: IRStmtType,
    pub content: String, // Serialized expression/details

// ================================================================================
// EXPRESSIONS
// ================================================================================

}
#[derive(Debug, Clone, PartialEq)]
pub enum IRExprType {
    Literal,
    Identifier,
    BinaryOp,
    UnaryOp,
    Call,
    Index,
    Attribute,
    Construct,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRExpression {
    pub base: IRNode,
    pub expr_type: IRExprType,
    pub value: String,
    pub children: Vec<IRExpression>,

// ================================================================================
// TYPES
// ================================================================================

}
#[derive(Debug, Clone, PartialEq)]
pub struct IRType {
    pub name: String,
    pub generics: Vec<IRType>,
    pub is_reference: bool,
    pub is_mutable: bool,
}