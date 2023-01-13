use std::{fmt::Debug, borrow::Borrow};

use crate::literal::Literal;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Ne,
    Eq,
    EqEq,
    Gt,
    Ge,
    Lt,
    Le,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    True,
    False,
    Fn,
    For,
    If,
    None,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}


pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] ({}) {}", self.line, self.kind, self.lexeme)
    }
}
