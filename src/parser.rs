use std::{error::Error, fmt::Display};

use crate::{
    ast::expr::Expr,
    literal::Literal,
    lox::Lox,
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    lox: Lox,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, lox: Lox) -> Self {
        Self {
            current: 0,
            tokens,
            lox,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expr().ok()
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        if let Ok(e) = expr.clone() {
            while self.is([TokenKind::Ne, TokenKind::EqEq]) {
                let operator = self.prev();
                let right = self.comparison();
                if let Ok(r) = right {
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r))
                }
            }
        }

        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        if let Ok(e) = expr.clone() {
            while self.is([TokenKind::Gt, TokenKind::Ge, TokenKind::Lt, TokenKind::Le]) {
                let operator = self.prev();
                let right = self.term();
                if let Ok(r) = right {
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r))
                }
            }
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();

        if let Ok(e) = expr.clone() {
            while self.is([TokenKind::Minus, TokenKind::Plus]) {
                let operator = self.prev();
                let right = self.factor();

                if let Ok(r) = right {
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r))
                }
            }
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();

        if let Ok(e) = expr.clone() {
            while self.is([TokenKind::Slash, TokenKind::Star]) {
                let operator = self.prev();

                let right = self.unary();
                if let Ok(ref r) = right {
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r));
                }
            }
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.is([TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.prev();
            let right = self.unary();
            if let Ok(r) = right {
                return Ok(Expr::Unary(operator.unwrap(), !r));
            } else {
                return right;
            }
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.is([TokenKind::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }

        if self.is([TokenKind::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }

        if self.is([TokenKind::False]) {
            return Ok(Expr::Literal(Literal::None));
        }

        if self.is([TokenKind::Number, TokenKind::String]) {
            return Ok(Expr::Literal(self.prev().unwrap().literal));
        }

        if self.is([TokenKind::LeftParen]) {
            let expr = self.expr();
            self.consume(TokenKind::RightParen, "Expected ')'".to_string());
            if let Ok(e) = expr {
                return Ok(Expr::Grouping(!e));
            }

            return expr;
        }

        let pk = self.peek();
        Err(self.error(pk, "Expected expression".to_string()))
    }

    fn sync(&mut self) {
        self.next();

        while !self.is_at_end() {
            if let Some(x) = self.prev() && x.kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fn
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print => return,
                _ => {}
            }
        }

        self.next();
    }

    fn consume(&mut self, kind: TokenKind, message: String) -> Token {
        if self.check(kind) {
            return self.next().unwrap();
        }

        let p = self.peek();
        panic!("{}", self.error(p, message))
    }

    fn error(&mut self, token: Token, message: String) -> ParseError {
        self.lox.error_on(token, message);
        return ParseError();
    }

    fn is<const T: usize>(&mut self, types: [TokenKind; T]) -> bool {
        for kind in types {
            if self.check(kind) {
                self.next();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().kind == kind;
    }

    fn next(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.prev();
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn prev(&self) -> Option<Token> {
        self.current
            .checked_sub(1)
            .map(|x| self.tokens.get(x).unwrap())
            .cloned()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError();
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}
