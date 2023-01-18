use std::collections::HashMap;

use crate::{ literal::Literal, everest::Everest, token::{ Token, TokenKind } };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    lox: Everest,
}

impl Scanner {
    pub fn new(source: String, lox: Everest) -> Self {
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
        // println!("{}", self.source);
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".to_string(), Literal::None, self.line));

        &self.tokens
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.put_token(TokenKind::LeftParen),
            ')' => self.put_token(TokenKind::RightParen),
            '{' => self.put_token(TokenKind::LeftBrace),
            '}' => self.put_token(TokenKind::RightBrace),
            ',' => self.put_token(TokenKind::Comma),
            '.' => self.put_token(TokenKind::Dot),
            '-' => self.put_token(TokenKind::Minus),
            '+' => self.put_token(TokenKind::Plus),
            ';' => self.put_token(TokenKind::Semicolon),
            '*' => self.put_token(TokenKind::Star),
            '!' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenKind::Ne)
                    .unwrap_or(TokenKind::Bang);
                self.put_token(tok);
            }
            '=' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenKind::EqEq)
                    .unwrap_or(TokenKind::Eq);
                self.put_token(tok);
            }
            '<' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenKind::Le)
                    .unwrap_or(TokenKind::Lt);
                self.put_token(tok);
            }
            '>' => {
                let tok = self
                    .is_next('=')
                    .then(|| TokenKind::Ge)
                    .unwrap_or(TokenKind::Gt);
                self.put_token(tok);
            }
            '/' => {
                if self.is_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.put_token(TokenKind::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),

            c if c.is_ascii_digit() => {
                self.number();
            }

            c if c.is_alphabetic() => self.identifier(),

            c => self.lox.error(self.line, format!("Unexpected character '{c}'")),
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

    fn add_token(&mut self, kind: TokenKind, literal: Literal) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token::new(kind, text.to_string(), literal, self.line))
    }

    fn put_token(&mut self, kind: TokenKind) {
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
        self.add_token(TokenKind::String, Literal::String(value.to_string()));
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

        let mov = &self.source[self.start..self.current];

        self.add_token(TokenKind::Number, Literal::Number(mov.parse().unwrap()))
    }

    pub fn peek_next(&mut self) -> char {
        self.source
            .chars()
            .nth(self.current + 1)
            .unwrap_or('\0')
    }

    pub fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let h = Self::keywords();
        let kind = h.get(text).unwrap_or(&TokenKind::Identifier);

        self.put_token(*kind)
    }

    pub fn keywords() -> HashMap<String, TokenKind> {
        let mut hash = HashMap::new();

        hash.insert("and", TokenKind::And);
        hash.insert("class", TokenKind::Class);
        hash.insert("else", TokenKind::Else);
        hash.insert("false", TokenKind::False);
        hash.insert("for", TokenKind::For);
        hash.insert("fun", TokenKind::Fn);
        hash.insert("if", TokenKind::If);
        hash.insert("nil", TokenKind::None);
        hash.insert("or", TokenKind::Or);
        hash.insert("print", TokenKind::Print);
        hash.insert("return", TokenKind::Return);
        hash.insert("super", TokenKind::Super);
        hash.insert("this", TokenKind::This);
        hash.insert("true", TokenKind::True);
        hash.insert("var", TokenKind::Var);
        hash.insert("while", TokenKind::While);

        hash.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }
}