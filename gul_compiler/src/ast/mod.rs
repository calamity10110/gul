// AST Node Definitions

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    List(Box<Type>),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Var {
        name: String,
        value: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    SetIndex {
        target: Expression,
        index: Expression,
        value: Expression,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return {
        value: Option<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Loop {
        body: Vec<Statement>,
    },
    Match {
        value: Expression,
        arms: Vec<MatchArm>,
    },
    Break,
    Continue,
    Expression {
        expr: Expression,
    },
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(ExpressionKind),
    Identifier(String),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub ty: Type,
}

// Need PartialEq for Pattern matching in some contexts, but not strictly required by AST consumer yet.
#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    List(Vec<Expression>),
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}
