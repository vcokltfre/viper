use colored::Colorize;
use std::fmt;

use super::super::lexer::*;
use super::ast::*;

#[derive(Debug)]
pub struct ParsingError {
    pub line: u32,
    pub column: u32,
    pub index: u32,
    pub filename: String,
    pub message: String,
    pub line_context: String,
    pub token_size: u32,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut padding = String::new();
        if self.column - 1 - self.token_size > 0 {
            padding = " ".repeat((self.column - 1 - self.token_size) as usize);
        }
        padding.push_str(&"~".repeat(self.token_size as usize));

        let arrow = "-->".blue().bold();

        write!(
            f,
            "Parsing failed: {}\n {} {}:{}:{} ({})\n\n   {}\n   {}",
            self.message,
            arrow,
            self.filename,
            self.line,
            self.column,
            self.index,
            self.line_context,
            padding.yellow().bold(),
        )
    }
}

impl ParsingError {
    pub fn new(at: &Token, message: String, line: String) -> ParsingError {
        ParsingError {
            line: at.line,
            column: at.column,
            index: at.index,
            filename: at.filename.clone(),
            message: message,
            line_context: line,
            token_size: at.length,
        }
    }
}

type ParsingResult<T> = Result<T, ParsingError>;

pub struct Parser {
    tokens: Vec<Token>,
    lines: Vec<String>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, lines: Vec<String>) -> Parser {
        Parser {
            tokens: tokens,
            lines: lines,
            index: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn error(&self, message: String) -> ParsingError {
        let token = &self.tokens[self.index];
        let line = self.lines[(token.line - 1) as usize].clone();

        ParsingError::new(token, message, line)
    }

    fn get_stmt(&mut self) -> ParsingResult<StmtNode> {
        let token = &self.tokens[self.index];

        match token.token_type {
            _ => {
                return Err(
                    self.error("Unexpected token: ".to_string() + &token.token_type.to_string())
                )
            }
        }
    }

    pub fn parse(&mut self) -> ParsingResult<AST> {
        let mut ast = AST::new();

        while !self.is_done() {
            ast.nodes.push(self.get_stmt()?);
        }

        Ok(ast)
    }
}
