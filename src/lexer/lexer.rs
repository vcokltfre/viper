use std::fmt;

use super::{Token, TokenType};

#[derive(Debug)]
pub struct TokenisationError {
    pub line: u32,
    pub column: u32,
    pub index: u32,
    pub filename: String,
    pub message: String,
    pub line_context: String,
}

impl fmt::Display for TokenisationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut padding = String::new();
        if self.column - 1 > 0 {
            padding = " ".repeat((self.column - 1) as usize);
        }
        padding.push('^');

        write!(
            f,
            "Failed to tokenise file \"{}\" [{};{}] ({}): {}\n\n{}\n{}",
            self.filename,
            self.line,
            self.column,
            self.index,
            self.message,
            self.line_context,
            padding,
        )
    }
}

type TokenisationResult = Result<Token, TokenisationError>;

pub struct Lexer {
    pub filename: String,
    pub source: String,
    pub index: u32,
    pub line: u32,
    pub column: u32,
    pub lines: Vec<String>,
}

impl Lexer {
    pub fn new(filename: String, source: String) -> Lexer {
        let lines = source.clone().lines().map(|s| s.to_string()).collect();

        Lexer {
            filename,
            source,
            index: 0,
            line: 1,
            column: 1,
            lines: lines,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.index as usize);
        if c.is_some() {
            self.index += 1;
            self.column += 1;
        }
        c
    }

    fn peek(&self, offset: u32) -> Option<char> {
        self.source.chars().nth((self.index + offset) as usize)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.index >= self.source.len() as u32
    }

    fn is_boundary(&self) -> bool {
        let c = self.peek(0);
        if c.is_none() {
            return true;
        }

        match c.unwrap() {
            ' ' | '\t' | '\r' | '\n' | '(' | ')' | '{' | '}' | '=' | '+' | '-' | '*' | '/'
            | '%' | '^' | ',' | '.' | '!' | '>' | '<' | '&' | '|' => true,
            _ => false,
        }
    }

    fn error(&self, message: String) -> TokenisationError {
        TokenisationError {
            line: self.line,
            column: self.column,
            index: self.index,
            filename: self.filename.clone(),
            message,
            line_context: self.lines[(self.line - 1) as usize].clone(),
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            column: self.column,
            index: self.index,
            filename: self.filename.clone(),
        }
    }

    fn get_number(&mut self) -> TokenisationResult {
        let mut number = String::new();
        let mut is_float = false;

        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                '0'..='9' => {
                    number.push(c.unwrap());
                    self.advance();
                }
                '.' => {
                    let next = self.peek(1);
                    if next.is_some() {
                        if next.unwrap() == '.' {
                            break;
                        }
                    }

                    if is_float {
                        return Err(self.error(
                            "Illegal second decimal point in float literal: '.'".to_string(),
                        ));
                    }
                    number.push(c.unwrap());
                    self.advance();
                    is_float = true;
                }
                _ => {
                    break;
                }
            }
        }

        if !self.is_boundary() {
            let c = self.peek(0).unwrap().to_string();
            return Err(self.error("Unexpected character in numeric literal: ".to_string() + &c));
        }

        if is_float {
            Ok(self.make_token(TokenType::Float(number.parse().unwrap())))
        } else {
            Ok(self.make_token(TokenType::Int(number.parse().unwrap())))
        }
    }

    fn get_ident(&mut self) -> TokenisationResult {
        let mut ident = String::new();

        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    ident.push(c.unwrap());
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        if !self.is_boundary() {
            let c = self.peek(0).unwrap().to_string();
            return Err(self.error("Unexpected character in identifier: ".to_string() + &c));
        }

        match ident.as_str() {
            "if" => return Ok(self.make_token(TokenType::KWIf)),
            "else" => return Ok(self.make_token(TokenType::KWElse)),
            "for" => return Ok(self.make_token(TokenType::KWFor)),
            "return" => return Ok(self.make_token(TokenType::KWReturn)),
            "break" => return Ok(self.make_token(TokenType::KWBreak)),
            "continue" => return Ok(self.make_token(TokenType::KWContinue)),
            "in" => return Ok(self.make_token(TokenType::KWIn)),
            "true" => return Ok(self.make_token(TokenType::Bool(true))),
            "false" => return Ok(self.make_token(TokenType::Bool(false))),
            _ => Ok(self.make_token(TokenType::Ident(ident))),
        }
    }

    fn get_single(&mut self) -> TokenisationResult {
        let c = self.peek(0).unwrap();
        self.advance();

        match c {
            '+' => Ok(self.make_token(TokenType::OpAdd)),
            '-' => Ok(self.make_token(TokenType::OpSub)),
            '*' => Ok(self.make_token(TokenType::OpMul)),
            '/' => Ok(self.make_token(TokenType::OpDiv)),
            '%' => Ok(self.make_token(TokenType::OpMod)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '!' => Ok(self.make_token(TokenType::OpNot)),
            '=' => Ok(self.make_token(TokenType::OpAssign)),
            '<' => Ok(self.make_token(TokenType::OpLt)),
            '>' => Ok(self.make_token(TokenType::OpGt)),
            '(' => Ok(self.make_token(TokenType::LParen)),
            ')' => Ok(self.make_token(TokenType::RParen)),
            '{' => Ok(self.make_token(TokenType::LBrace)),
            '}' => Ok(self.make_token(TokenType::RBrace)),

            _ => Err(self.error("Unexpected character: ".to_string() + &c.to_string())),
        }
    }

    fn get_multi(&mut self) -> TokenisationResult {
        let c = self.peek(0).unwrap();
        let next = self.peek(1);

        if next.is_none() {
            return self.get_single();
        }

        let next_c = next.unwrap();

        match (c, next_c) {
            ('*', '*') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpPow))
            }
            ('=', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpEq))
            }
            ('>', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpGe))
            }
            ('<', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpLe))
            }
            ('!', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpNe))
            }
            ('&', '&') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpAnd))
            }
            ('|', '|') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::OpOr))
            }
            ('=', '>') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::Arrow))
            }
            ('.', '.') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::Range))
            }

            _ => self.get_single(),
        }
    }

    fn get_string(&mut self) -> TokenisationResult {
        let mut value = String::new();

        self.advance();

        let mut escape = false;

        loop {
            let c = self.advance();

            if c.is_none() {
                return Err(self.error("Unterminated string literal".to_string()));
            }

            let c = c.unwrap();

            if escape {
                match c {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '0' => value.push('\0'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    _ => {
                        return Err(
                            self.error("Invalid escape sequence: \\".to_string() + &c.to_string())
                        )
                    }
                }
                escape = false;
                continue;
            }

            if c == '\\' {
                escape = true;
                continue;
            }

            if c == '"' {
                break;
            }

            value.push(c);
        }

        Ok(self.make_token(TokenType::String(value)))
    }

    fn get_token(&mut self) -> TokenisationResult {
        self.skip_whitespace();

        if self.is_end() {
            return Ok(self.make_token(TokenType::EOF));
        }

        let c = self.peek(0).unwrap();

        match c {
            '-' => {
                let c = self.peek(1);
                if c.is_some() && c.unwrap().is_ascii_digit() {
                    self.advance();
                    let mut token = self.get_number()?;
                    match token.token_type {
                        TokenType::Int(i) => token.token_type = TokenType::Int(-i),
                        TokenType::Float(f) => token.token_type = TokenType::Float(-f),
                        _ => {}
                    }
                    return Ok(token);
                }
                self.advance();
                Ok(self.make_token(TokenType::OpSub))
            }
            '+' | '*' | '/' | '%' | ',' | '.' | '!' | '=' | '<' | '>' | '&' | '|' | '(' | ')'
            | '{' | '}' => self.get_multi(),
            '0'..='9' => self.get_number(),
            'a'..='z' | 'A'..='Z' => self.get_ident(),
            '"' => self.get_string(),
            _ => Err(self.error("Unexpected character: ".to_string() + &c.to_string())),
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, TokenisationError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.get_token()?;
            if token.token_type == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}
