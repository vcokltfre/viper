use super::super::lexer::*;
use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        Ok(AST::new())
    }
}
