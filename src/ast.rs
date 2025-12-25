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
    // GUL 101 Ownership Modes (current)
    Own,    // Exactly one owner, does not move (use in node contracts)
    Borrow, // Temporary mutable access, no ownership move
    Ref,    // Shared immutable access, no ownership move
    Take,   // Ownership transfer (move)
    Gives,  // Ownership transfer via copy

    // DEPRECATED - use Gives instead
    #[deprecated(since = "0.14.0", note = "use Gives instead")]
    Copy,
}

impl Ownership {
    /// Check if this mode moves ownership
    pub fn moves_ownership(&self) -> bool {
        matches!(self, Ownership::Take | Ownership::Gives)
    }

    /// Check if this mode allows mutation
    pub fn is_mutable(&self) -> bool {
        matches!(self, Ownership::Own | Ownership::Borrow | Ownership::Take)
    }

    /// Check if this mode copies data
    #[allow(deprecated)]
    pub fn copies_data(&self) -> bool {
        matches!(self, Ownership::Gives | Ownership::Copy)
    }

    /// Parse from string
    #[allow(deprecated)]
    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "own" => Some(Ownership::Own),
            "borrow" => Some(Ownership::Borrow),
            "ref" => Some(Ownership::Ref),
            "take" => Some(Ownership::Take),
            "gives" => Some(Ownership::Gives),
            "copy" => Some(Ownership::Gives), // Map deprecated 'copy' to 'gives'
            _ => None,
        }
    }
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

// ============================================
// Data-Flow Node System (v3.0)
// ============================================

/// Type annotation for data-flow contracts (e.g., @int, @float, @str)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub base_type: String,   // e.g., "int", "float", "str"
    pub is_list: bool,       // @int[] vs @int
    pub traits: Vec<String>, // Associated traits
}

/// Port parameter with name and label
#[derive(Debug, Clone, PartialEq)]
pub struct PortParam {
    pub name: String,  // Parameter name (lowercase)
    pub label: String, // Label (uppercase)
}

/// Port contract for node input/output
#[derive(Debug, Clone, PartialEq)]
pub struct PortContract {
    pub port_type: TypeAnnotation,
    pub params: Vec<PortParam>,
}

/// Node declaration (in .def files)
#[derive(Debug, Clone, PartialEq)]
pub struct NodeDeclaration {
    pub name: String,
    pub required_inputs: PortContract,
    pub required_outputs: PortContract,
    pub optional_inputs: Option<PortContract>,
    pub optional_outputs: Option<PortContract>,
}

/// Reference to a node port (e.g., node1: c)
#[derive(Debug, Clone, PartialEq)]
pub struct PortRef {
    pub node_name: String, // "node1" or "input" or "print"
    pub port_name: String, // "c" or "a"
}

/// Data-flow connection (source : target)
#[derive(Debug, Clone, PartialEq)]
pub struct DataFlowConnection {
    pub source: PortRef,
    pub target: PortRef,
}

/// External input with type and value
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalInput {
    pub input_type: TypeAnnotation,
    pub value: Expression,
}

/// Data-flow main block
#[derive(Debug, Clone, PartialEq)]
pub struct DataFlowBlock {
    pub connections: Vec<DataFlowConnection>,
    pub external_inputs: Vec<(ExternalInput, PortRef)>,
    pub outputs: Vec<PortRef>, // Ports connected to print/output
}

/// Trait definition
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDef {
    pub name: String,
    pub methods: Vec<String>, // Optional trait methods
}

/// Node function implementation output binding
#[derive(Debug, Clone, PartialEq)]
pub struct OutputBinding {
    pub name: String,           // Output parameter name
    pub expression: Expression, // The computation
}

/// Node function return with multiple outputs
#[derive(Debug, Clone, PartialEq)]
pub struct NodeReturn {
    pub bindings: Vec<OutputBinding>,
}

// Add to Statement enum (extend existing)
// These are for reference - actual integration in parser

/// Built-in traits for the data-flow system
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinTrait {
    Serialize,
    Trainable,
    Stream,
    Async,
    Sync,
}

impl std::str::FromStr for BuiltinTrait {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "serialize" => Ok(BuiltinTrait::Serialize),
            "trainable" => Ok(BuiltinTrait::Trainable),
            "stream" => Ok(BuiltinTrait::Stream),
            "async" => Ok(BuiltinTrait::Async),
            "sync" => Ok(BuiltinTrait::Sync),
            _ => Err(()),
        }
    }
}

impl TypeAnnotation {
    pub fn new(base_type: &str) -> Self {
        TypeAnnotation {
            base_type: base_type.to_string(),
            is_list: false,
            traits: Vec::new(),
        }
    }

    pub fn with_traits(base_type: &str, traits: Vec<String>) -> Self {
        TypeAnnotation {
            base_type: base_type.to_string(),
            is_list: false,
            traits,
        }
    }
}

impl PortRef {
    pub fn new(node_name: &str, port_name: &str) -> Self {
        PortRef {
            node_name: node_name.to_string(),
            port_name: port_name.to_string(),
        }
    }
}
