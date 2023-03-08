#[derive(Debug, PartialEq)]
pub enum TokenType {
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    OpAssign, // =

    OpAdd, // +
    OpSub, // -
    OpMul, // *
    OpDiv, // /
    OpMod, // %
    OpPow, // **

    OpEq, // ==
    OpNe, // !=
    OpLt, // <
    OpLe, // <=
    OpGt, // >
    OpGe, // >=

    OpAnd, // &&
    OpOr,  // ||
    OpNot, // !

    Comma, // ,
    Dot,   // .
    Arrow, // =>
    Range, // ..

    KWIf,       // if
    KWElse,     // else
    KWFor,      // for
    KWReturn,   // return
    KWBreak,    // break
    KWContinue, // continue
    KWIn,       // in

    Ident(String),
    Int(i64),
    Float(f64),
    String(String),

    Bool(bool),

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub column: u32,
    pub index: u32,
    pub filename: String,
}
