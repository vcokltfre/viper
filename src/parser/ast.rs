#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum ExprNode {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Ident(String),
    Binary(Operator, Box<ExprNode>, Box<ExprNode>),
    Unary(Operator, Box<ExprNode>),
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub typ: String,
}

#[derive(Debug)]
pub enum StmtNode {
    Expr(ExprNode),
    Assignment(String, ExprNode),
    Return(ExprNode),
    Break,
    Continue,

    // Cond, body, else
    If(ExprNode, Vec<StmtNode>, Vec<StmtNode>),

    // Var, cond, body
    For(String, ExprNode, Vec<StmtNode>),

    // Name, params, ret, body
    Function(String, Vec<FunctionParameter>, String, Vec<StmtNode>),

    // Filename, line
    Context(String, u32),
}

#[derive(Debug)]
pub struct AST {
    pub nodes: Vec<StmtNode>,
}

impl AST {
    pub fn new() -> AST {
        AST { nodes: Vec::new() }
    }
}
