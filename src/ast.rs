// AST (Abstract Syntax Tree) definitions

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Any,                            // Gradual typing - can hold any value
    Unit(String),                   // Unit types like "m", "kg", "m/s"
    Function(Vec<Type>, Box<Type>), // Parameter types and return type
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Import(String),
    StructDef {
        name: String,
        fields: Vec<(String, String)>,
        methods: Vec<Statement>,
    },
    Definition {
        name: String,
        value: Expression,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        is_async: bool,
    },
    ForeignBlock {
        language: String,
        code: String,
    },
    GlobalDef {
        name: String,
        value: Expression,
        mutable: bool,
    },
    Main {
        body: Vec<Statement>,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Expression(Expression),
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Loop {
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
    Try {
        try_body: Vec<Statement>,
        catch_var: Option<String>,
        catch_body: Option<Vec<Statement>>,
        finally_body: Option<Vec<Statement>>,
    },
    Throw(Expression),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Await(Box<Expression>),
    UiSprite {
        sprite_type: String,
        properties: Vec<(String, Expression)>,
    },
    List(Vec<Expression>),
    Dict(Vec<(String, Expression)>),
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Member {
        object: Box<Expression>,
        member: String,
    },
    Ownership {
        mode: Ownership,
        value: Box<Expression>,
    },
    ListOp {
        op: ListOp,
        args: Vec<Expression>,
    },
    // Type-annotated expressions for gradual typing
    Typed {
        expr: Box<Expression>,
        ty: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ownership {
    Own,
    Ref,
    Copy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListOp {
    Car,
    Cdr,
    Cons,
    Map,
    Fold,
    Slice,
}
