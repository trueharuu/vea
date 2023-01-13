use std::collections::HashMap;

use crate::{
    literal::Literal,
    lox::Lox,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    lox: Lox,
}

impl Scanner {
    pub fn new(source: String, lox: Lox) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
            line: 0,
            start: 0,
            lox,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::None,
            self.line,
        ));

        &self.tokens
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.put_token(TokenType::LeftParen),
            ')' => self.put_token(TokenType::RightParen),
            '{' => self.put_token(TokenType::LeftBrace),
            '}' => self.put_token(TokenType::RightBrace),
            ',' => self.put_token(TokenType::Comma),
            '.' => self.put_token(TokenType::Dot),
            '-' => self.put_token(TokenType::Minus),
            '+' => self.put_token(TokenType::Plus),
            ';' => self.put_token(TokenType::Semicolon),
            '*' => self.put_token(TokenType::Star),
            '!' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenType::Ne)
                    .unwrap_or(TokenType::Bang);
                self.put_token(tok);
            }
            '=' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenType::EqEq)
                    .unwrap_or(TokenType::Eq);
                self.put_token(tok);
            }
            '<' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenType::Le)
                    .unwrap_or(TokenType::Lt);
                self.put_token(tok);
            }
            '>' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenType::Ge)
                    .unwrap_or(TokenType::Gt);
                self.put_token(tok);
            }
            '/' => {
                if self.is_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.put_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }

            // math symbols
            '≠' => self.put_token(TokenType::Ne),
            '≤' => self.put_token(TokenType::Le),
            '≥' => self.put_token(TokenType::Ge),

            '"' => self.string(),

            c if c.is_ascii_digit() => {
                self.number();
            }

            c if c.is_alphabetic() => self.identifier(),

            c => self
                .lox
                .error(self.line, format!("Unexpected character '{c}'")),
        }
    }

    fn advance(&mut self) -> char {
        let i = {
            let temp = self.current;
            self.current += 1;
            temp
        };

        self.source[i..i + 1].chars().next().unwrap()
    }

    fn add_token(&mut self, kind: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(kind, text.to_string(), literal, self.line))
    }

    fn put_token(&mut self, kind: TokenType) {
        self.add_token(kind, Literal::None);
    }

    fn is_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, format!("Unterminated string"));
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Literal::Number(self.source[self.start..self.current].parse().unwrap()),
        )
    }

    pub fn peek_next(&mut self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    pub fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let h = Self::keywords();
        let kind = h.get(text).unwrap_or(&TokenType::Identifier);

        self.put_token(*kind)
    }

    pub fn keywords() -> HashMap<String, TokenType> {
        let mut hash = HashMap::new();

        hash.insert("and", TokenType::And);
        hash.insert("class", TokenType::Class);
        hash.insert("else", TokenType::Else);
        hash.insert("false", TokenType::False);
        hash.insert("for", TokenType::For);
        hash.insert("fun", TokenType::Fn);
        hash.insert("if", TokenType::If);
        hash.insert("nil", TokenType::None);
        hash.insert("or", TokenType::Or);
        hash.insert("print", TokenType::Print);
        hash.insert("return", TokenType::Return);
        hash.insert("super", TokenType::Super);
        hash.insert("this", TokenType::This);
        hash.insert("true", TokenType::True);
        hash.insert("var", TokenType::Var);
        hash.insert("while", TokenType::While);

        hash.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }
}
