// GUL v3.2 Compiler - AST Node Definitions (Refactored)
use std::collections::{HashMap, HashSet};
use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    pub line: usize,
    pub column: usize,

}
impl ASTNode {
    pub fn location(&self)  ->  String {
        return format!("{}:{}", self.line, self.column);

// Enums for Types
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral, NoneLiteral,
    ListLiteral, TupleLiteral, SetLiteral, DictLiteral,
    Identifier, BinaryOp, UnaryOp, Call, Index, Attribute,
    Lambda, MatchExpr, TypeConstructor, Grouped, Await, Table,

}
#[derive(Debug, Clone, PartialEq)]
pub enum StmtType {
    LetDecl, VarDecl, FunctionDecl, StructDecl, EnumDecl,
    IfStmt, WhileStmt, ForStmt, LoopStmt, MatchStmt,
    BreakStmt, ContinueStmt, ReturnStmt,
    TryStmt, ExpressionStmt, AssignmentStmt, ImportStmt, ForeignCodeBlock, PassStmt,

// Expression Enum
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Call(CallExpr),
    Index(IndexExpr),
    Attribute(AttributeExpr),
    List(ListExpr),
    Tuple(TupleExpr),
    Set(SetExpr),
    Dict(DictExpr),
    Lambda(LambdaExpr),
    Match(MatchExpr),
    TypeConstructor(TypeConstructorExpr), // @int(), @float()
    Grouped(GroupedExpr),
    Await(AwaitExpr),
    Table(TableExpr),

}

impl Expression {
    pub fn get_node(&self) -> ASTNode {
        match self {
            Expression::Literal(e) => e.node.clone(),
            Expression::Identifier(e) => e.node.clone(),
            Expression::BinaryOp(e) => e.node.clone(),
            Expression::UnaryOp(e) => e.node.clone(),
            Expression::Call(e) => e.node.clone(),
            Expression::Index(e) => e.node.clone(),
            Expression::Attribute(e) => e.node.clone(),
            Expression::List(e) => e.node.clone(),
            Expression::Tuple(e) => e.node.clone(),
            Expression::Set(e) => e.node.clone(),
            Expression::Dict(e) => e.node.clone(),
            Expression::Lambda(e) => e.node.clone(),
            Expression::Match(e) => e.node.clone(),
            Expression::TypeConstructor(e) => e.node.clone(),
            Expression::Grouped(e) => e.node.clone(),
            Expression::Await(e) => e.node.clone(),
            Expression::Table(e) => e.node.clone(),
        }
    }
}

// Statement Enum
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetDecl(LetStmt),
    VarDecl(VarStmt),
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    LoopStmt(LoopStmt),
    MatchStmt(MatchStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt),
    TryStmt(TryStmt),
    ExpressionStmt(ExpressionStmt),
    AssignmentStmt(AssignmentStmt),
    ImportStmt(ImportStmt),
    ForeignCodeBlock(ForeignCodeBlock),
    PassStmt(PassStmt),

// Implement basic methods for Statement
}
impl Statement {
    pub fn stmt_type(&mut self)  ->  StmtType {
        match self {
            Statement::LetDecl(s) => return StmtType::LetDecl,
            Statement::VarDecl(s) => return StmtType::VarDecl,
            Statement::FunctionDecl(s) => return StmtType::FunctionDecl,
            Statement::StructDecl(s) => return StmtType::StructDecl,
            Statement::EnumDecl(s) => return StmtType::EnumDecl,
            Statement::IfStmt(s) => return StmtType::IfStmt,
            Statement::WhileStmt(s) => return StmtType::WhileStmt,
            Statement::ForStmt(s) => return StmtType::ForStmt,
            Statement::LoopStmt(s) => return StmtType::LoopStmt,
            Statement::MatchStmt(s) => return StmtType::MatchStmt,
            Statement::BreakStmt(s) => return StmtType::BreakStmt,
            Statement::ContinueStmt(s) => return StmtType::ContinueStmt,
            Statement::ReturnStmt(s) => return StmtType::ReturnStmt,
            Statement::TryStmt(s) => return StmtType::TryStmt,
            Statement::ExpressionStmt(s) => return StmtType::ExpressionStmt,
            Statement::AssignmentStmt(s) => return StmtType::AssignmentStmt,
            Statement::ImportStmt(s) => return StmtType::ImportStmt,
            Statement::ForeignCodeBlock(s) => return StmtType::ForeignCodeBlock,
            Statement::PassStmt(s) => return StmtType::PassStmt,

// Expression Structs
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub node: ASTNode,
    pub value: String,
    pub value_type: TokenType,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierExpr {
    pub node: ASTNode,
    pub name: String,

}
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOpExpr {
    pub node: ASTNode,
    pub left: Box<Expression>,
    pub operator: TokenType,
    pub right: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpExpr {
    pub node: ASTNode,
    pub operator: TokenType,
    pub operand: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub node: ASTNode,
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub keyword_args: HashMap<String, String>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub node: ASTNode,
    pub object: Box<Expression>,
    pub index: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeExpr {
    pub node: ASTNode,
    pub object: Box<Expression>,
    pub attribute: String,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ListExpr {
    pub node: ASTNode,
    pub elements: Vec<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct TupleExpr {
    pub node: ASTNode,
    pub elements: Vec<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct SetExpr {
    pub node: ASTNode,
    pub elements: Vec<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct DictExpr {
    pub node: ASTNode,
    pub pairs: Vec<(Expression, Expression)>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpr {
    pub node: ASTNode,
    pub parameters: Vec<String>,
    pub body: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub node: ASTNode,
    pub value: Box<Expression>,
    pub cases: Vec<MatchCase>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Expression,
    pub guard: Expression,
    pub body: Expression,

}
#[derive(Debug, Clone, PartialEq)]
pub struct TypeConstructorExpr {
    pub node: ASTNode,
    pub type_name: String,
    pub argument: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct GroupedExpr {
    pub node: ASTNode,
    pub expression: Box<Expression>,

// Statements
}
#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub node: ASTNode,
    pub name: String,
    pub type_annotation: String,
    pub value: Expression,

}
#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub node: ASTNode,
    pub name: String,
    pub type_annotation: String,
    pub value: Expression,

}
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStmt {
    pub node: ASTNode,
    pub target: Expression,
    pub operator: TokenType,
    pub value: Expression,

}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub node: ASTNode,
    pub name: String,
    pub is_async: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub body: Vec<Statement>,
    pub decorators: Vec<String>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String, // Kept for backward compatibility, empty if pattern used
    pub pattern: Option<Expression>, // For destructuring: {x, y} or [a, b]
    pub type_annotation: String,
    pub ownership_mode: String,
    pub default_value: Option<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub node: ASTNode,
    pub name: String,
    pub fields: Vec<StructField>,
    pub methods: Vec<FunctionDecl>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub type_annotation: String,

}
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub node: ASTNode,
    pub name: String,
    pub variants: Vec<String>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub node: ASTNode,
    pub condition: Expression,
    pub then_body: Vec<Statement>,
    pub elif_clauses: Vec<ElifClause>,
    pub else_body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ElifClause {
    pub condition: Expression,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub node: ASTNode,
    pub condition: Expression,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub node: ASTNode,
    pub variable: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt {
    pub node: ASTNode,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct MatchStmt {
    pub node: ASTNode,
    pub value: Expression,
    pub cases: Vec<MatchStmtCase>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct MatchStmtCase {
    pub pattern: Expression,
    pub guard: Expression,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub node: ASTNode,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    pub node: ASTNode,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub node: ASTNode,
    pub value: Option<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct TryStmt {
    pub node: ASTNode,
    pub try_body: Vec<Statement>,
    pub catch_clauses: Vec<CatchClause>,
    pub finally_body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub exception_type: String,
    pub variable_name: String,
    pub body: Vec<Statement>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ImportStmt {
    pub node: ASTNode,
    pub module_path: Vec<String>,
    pub import_type: String,
    pub items: Vec<String>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignCodeBlock {
    pub node: ASTNode,
    pub language: String,
    pub code: String,

}
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt {
    pub node: ASTNode,
    pub expression: Expression,

}
#[derive(Debug, Clone, PartialEq)]
pub struct PassStmt {
    pub node: ASTNode,

}
#[derive(Debug, Clone, PartialEq)]
pub struct AwaitExpr {
    pub node: ASTNode,
    pub value: Box<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct TableExpr {
    pub node: ASTNode,
    pub columns: Vec<String>,
    pub rows: Vec<TableRow>,
    pub is_sparse: bool,

}
#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub name: String,
    pub values: Vec<Expression>,

}
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub imports: Vec<ImportStmt>,
    pub main_entry: Vec<Statement>,

}
pub fn create_node(line: usize, column: usize)  ->  ASTNode {
    return ASTNode{line: line, column: column};
}