use crate::{
    ast::expr::Expr,
    literal::Literal,
    token::{Token, TokenType}, lox::Lox,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    lox: Lox
}

impl Parser {
    pub fn new(tokens: Vec<Token>, lox: Lox) -> Self {
        Self { current: 0, tokens, lox }
    }

    fn expression(&self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.is_next(vec![TokenType::Ne, TokenType::EqEq]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Expr::Binary(expr, *operator, right))
        }

        expr
    }

    fn comparison(&self) -> Box<Expr> {
        let mut expr = self.term();

        while self.is_next(vec![
            TokenType::Gt,
            TokenType::Lt,
            TokenType::Ge,
            TokenType::Le,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Expr::Binary(expr, *operator, right))
        }

        expr
    }

    fn term(&self) -> Box<Expr> {
        let mut expr = self.factor();

        while self.is_next(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Expr::Binary(expr, *operator, right))
        }

        expr
    }

    fn factor(&self) -> Box<Expr> {
        let expr = self.unary();

        while self.is_next(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Expr::Binary(expr, *operator, right))
        }

        expr
    }

    fn unary(&self) -> Box<Expr> {
        if self.is_next(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Expr::Unary(*operator, right));
        }

        self.primary()
    }

    fn primary(&self) -> Box<Expr> {
        if self.is_next(vec![TokenType::False]) {
            return Box::new(Expr::Literal(Literal::Boolean(false)));
        }

        if self.is_next(vec![TokenType::True]) {
            return Box::new(Expr::Literal(Literal::Boolean(true)));
        }

        if self.is_next(vec![TokenType::None]) {
            return Box::new(Expr::Literal(Literal::None));
        }

        if self.is_next(vec![TokenType::Number, TokenType::String]) {
            return Box::new(Expr::Literal(self.previous().literal));
        }

        if self.is_next(vec![TokenType::LeftParen]) {
            let expr = self.expression();

            self.consume(TokenType::RightParen, "Expected ')' after expression".to_string());
            return Box::new(Expr::Grouping(expr));
        }

        unreachable!()
    }

    fn consume(&self, kind: TokenType, message: String) -> &Token {
      if self.check(kind) {
        return self.advance();
      }

      panic!("{}", self.error(self.peek(), message));
    }

    fn error(&self, token: Token, message: String) -> ParseError {
      self.lox.error(token, message);
      ParseError::new
    }

    fn is_next(&self, kinds: Vec<TokenType>) -> bool {
        for c in kinds {
            if self.check(c) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::new(
            TokenType::Eof,
            "\0".to_string(),
            Literal::None,
            0,
        ))
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap_or(&Token::new(
            TokenType::Eof,
            "\0".to_string(),
            Literal::None,
            0,
        ))
    }
}
