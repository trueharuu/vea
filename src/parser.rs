use std::{error::Error, fmt::Display};

use crate::{
    ast::{expr::Expr, statement::Stmt},
    everest::Everest,
    literal::Literal,
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    lox: Everest,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, lox: Everest) -> Self {
        Self {
            current: 0,
            tokens,
            lox,
        }
    }

    // pub fn parse(&mut self) -> Option<Vec<Stmt>> {
    //     let statements = Vec::new();

    //     while !self.is_at_end() {
    //         statements.push(self.statement())
    //     }
    // }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let result = self.declaration();
            if let Some(r) = result {
                statements.push(r);
            } else {
                return None;
            }
        }

        Some(statements)
    }

    fn declaration(&mut self) -> Option<Stmt> {
        // if self.is([TokenKind::Fn]) {
        //     return self.function("fn".to_string());
        // }

        let r = if self.is([TokenKind::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if let Ok(n) = r {
            Some(n)
        } else {
            self.sync();
            None
        }
    }

    fn function(&mut self, kind: String) -> Option<Stmt> {
        // let name = self
        //     .consume(TokenKind::Identifier, format!("expected {kind} name"))
        //     .ok();

        // if name.is_none() {
        //     return None;
        // }

        // let l = self
        //     .consume(
        //         TokenKind::LeftParen,
        //         format!("expected '(' after {kind} name"),
        //     )
        //     .ok();

        // if l.is_none() {
        //     return None;
        // }

        // let r = self
        //     .consume(
        //         TokenKind::RightParen,
        //         "expected ')' after fn params".to_string(),
        //     )
        //     .ok();

        // if r.is_none() {
        //     return None;
        // }

        // let k = self
        //     .consume(
        //         TokenKind::LeftBrace,
        //         format!("expected '{{' before {kind} body"),
        //     )
        //     .ok();

        // if k.is_none() {
        //     return None;
        // }

        // let body = self.block();

        // return Some(Stmt::Fn(name.unwrap(), Vec::new(), body));
        todo!();
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let n = self.consume(TokenKind::Identifier, "expect variable name".to_string());
        if let Ok(name) = &n {
            let mut initializer = None;

            if self.is([TokenKind::Eq]) {
                let expr = self.expr();
                if let Ok(e) = expr {
                    initializer = Some(e);
                }
            }

            self.consume(
                TokenKind::Semicolon,
                "expect ';' after variable declaration".to_string(),
            );
            // if initializer.is_none() {
            // return Err(self.error(name.clone(), format!("missing initializer for {}", name)));
            // } else {
            return Ok(Stmt::Var(
                name.clone(),
                initializer.unwrap_or(Expr::Literal(Literal::None)),
            ));
            // }
        } else {
            Err(n.unwrap_err())
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.is([TokenKind::For]) {
            return self.for_statement();
        }

        if self.is([TokenKind::If]) {
            return self.if_statement();
        }

        if self.is([TokenKind::Print]) {
            return self.print_statement();
        }

        if self.is([TokenKind::While]) {
            return self.while_statement();
        }

        if self.is([TokenKind::LeftBrace]) {
            let x = self.block();
            return Ok(Stmt::Block(x));
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        let l = self.consume(TokenKind::LeftParen, "expected '(' after while".to_string());

        if l.is_err() {
            return Err(l.unwrap_err());
        }

        let initializer;
        if self.is([TokenKind::Semicolon]) {
            initializer = None;
        } else if self.is([TokenKind::Var]) {
            let v = self.var_declaration();
            if v.is_err() {
                return v;
            }

            initializer = Some(v.unwrap());
        } else {
            let e = self.expression_statement();
            if e.is_err() {
                return e;
            }

            initializer = Some(e.unwrap());
        }

        let mut cond = None;
        if !self.check(TokenKind::Semicolon) {
            let e = self.expr();
            if e.is_err() {
                return Err(e.unwrap_err());
            }

            cond = Some(e.unwrap());
        }

        let sc = self.consume(
            TokenKind::Semicolon,
            "expected ';' after loop condition".to_string(),
        );
        if sc.is_err() {
            return Err(sc.unwrap_err());
        }

        let mut incr = None;
        if !self.check(TokenKind::RightParen) {
            let e = self.expr();
            if e.is_err() {
                return Err(e.unwrap_err());
            }

            incr = Some(e.unwrap());
        }

        let r = self.consume(
            TokenKind::RightParen,
            "expected ')' after for clause".to_string(),
        );

        if r.is_err() {
            return Err(r.unwrap_err());
        }

        let mut body = self.statement();

        if let Some(i) = incr {
            body = body.map(|ok| Stmt::Block(vec![ok, Stmt::Expression(i)]));
        }

        if cond.is_none() {
            cond = Some(Expr::Literal(Literal::Boolean(true)));
        }

        body = body.map(|ok| Stmt::While(cond.unwrap(), Box::new(ok)));

        if let Some(n) = initializer {
            body = body.map(|ok| Stmt::Block(vec![n, ok]));
        }

        body
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let l = self.consume(TokenKind::LeftParen, "expected '(' after while".to_string());
        if l.is_err() {
            return Err(l.unwrap_err());
        }

        let condition = self.expr();

        if condition.is_err() {
            return Err(condition.unwrap_err());
        }

        let r = self.consume(
            TokenKind::RightParen,
            "expected ')' to close while".to_string(),
        );

        if r.is_err() {
            return Err(r.unwrap_err());
        }

        let body = self.statement();

        if body.is_err() {
            return body;
        }

        return Ok(Stmt::While(condition.unwrap(), Box::new(body.unwrap())));
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let l = self.consume(TokenKind::LeftParen, "expected '(' after if".to_string());

        if l.is_err() {
            return Err(l.unwrap_err());
        }

        let condition = self.expr();

        if condition.is_err() {
            return Err(condition.unwrap_err());
        }

        let r = self.consume(
            TokenKind::RightParen,
            "expected ')' to close if".to_string(),
        );

        if r.is_err() {
            return Err(r.unwrap_err());
        }

        let then = self.statement();
        if then.is_err() {
            return Err(then.unwrap_err());
        }

        let mut elsn = None;

        if self.is([TokenKind::Else]) {
            let parse = self.statement();

            if parse.is_err() {
                return Err(parse.unwrap_err());
            }

            elsn = Some(Box::new(parse.unwrap()));
        }

        Ok(Stmt::If(condition.unwrap(), !then.unwrap(), elsn))
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        self.consume(
            TokenKind::RightBrace,
            "expected '}' after block".to_string(),
        );

        return statements;
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expr();
        self.consume(TokenKind::Semicolon, "expected ';' after value".to_string());
        return value.map(|e| Stmt::Print(e));
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expr();
        self.consume(TokenKind::Semicolon, "expected ';' after value".to_string());
        return value.map(|e| Stmt::Expression(e));
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or();

        if expr.is_err() {
            return Err(expr.unwrap_err());
        }

        if self.is([TokenKind::Eq]) {
            // assume we have something before this.
            let eq = self.prev().unwrap();
            let va = self.assignment();

            if let Ok(v) = va.clone() && let Ok(e) = expr.clone() {
                if let Expr::Variable(name) = e {
                    return Ok(Expr::Assign(name, !v));
                } else {
                    self.error(eq.clone(), format!("invalid assignment target '{eq}'"));
                }
            } else {
                return va;
            }
        }

        expr
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and();

        if expr.is_err() {
            return expr;
        }

        while self.is([TokenKind::Or]) {
            let op = self.prev().unwrap();
            let right = self.and();
            if right.is_err() {
                return right;
            }
            expr = Ok(Expr::Logical(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality();
        if expr.is_err() {
            return expr;
        }

        while self.is([TokenKind::And]) {
            let op = self.prev().unwrap();
            let right = self.equality();

            if right.is_err() {
                return Err(right.unwrap_err());
            }

            expr = Ok(Expr::Logical(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        if let Ok(e) = expr.clone() {
            while self.is([TokenKind::Ne, TokenKind::EqEq]) {
                let operator = self.prev();
                let right = self.comparison();
                if let Ok(r) = right {
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r));
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
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r));
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
                    expr = Ok(Expr::Binary(!e.clone(), operator.unwrap(), !r));
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

        return self.call();
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary();

        if expr.is_err() {
            return expr;
        }

        loop {
            if self.is([TokenKind::LeftParen]) {
                expr = self.finish_call(&expr.unwrap());
            } else {
                break;
            }
        }

        return expr;
    }

    fn finish_call(&mut self, callee: &Expr) -> Result<Expr, ParseError> {
        let mut args = Vec::new();
        if !self.check(TokenKind::LeftParen) {
            let mut first = false;
            let pk = self.peek();
            while !first || self.is([TokenKind::Comma]) {
                if args.len() >= 255 {
                    return Err(self.error(pk, "can't have more than 255 args for fn".to_string()));
                }
                let expr = self.expr();
                if let Ok(e) = expr {
                    args.push(Box::new(e));
                } else {
                    return expr;
                }

                first = true;
            }
        }

        let paren = self.consume(
            TokenKind::RightParen,
            "expected ')' after fn arguments".to_string(),
        );

        if paren.is_err() {
            return Err(paren.unwrap_err());
        }

        return Ok(Expr::Call(Box::new(callee.clone()), paren.unwrap(), args));
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

        if self.is([TokenKind::Identifier]) {
            return Ok(Expr::Variable(self.prev().unwrap()));
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
                | TokenKind::Print => {
                    return;
                }
                _ => {}
            }
        }

        self.next();
    }

    fn consume(&mut self, kind: TokenKind, message: String) -> Result<Token, ParseError> {
        if self.check(kind) {
            return Ok(self.next().unwrap());
        }

        let p = self.peek();
        Err(self.error(p, message))
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
        write!(f, "{self:?}")
    }
}

impl Error for ParseError {}
