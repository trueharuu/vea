use crate::lexer::Token;
use crate::literal::Literal;
use crate::span::Span;

#[derive(Default, Clone)]
pub enum Expr<'a> {
    Access {
        ident: Span<&'a str>,
    },

    Literal {
        value: Literal,
    },

    Group {
        left_paren: Token<'a>,
        expr: Box<Span<Self>>,
        right_paren: Token<'a>,
    },

    Let {
        let_token: Token<'a>,
        ident: Span<&'a str>,
        eq_token: Token<'a>,
        expr: Box<Span<Self>>,
        semi_token: Token<'a>,
    },

    Not {
        bang_token: Token<'a>,
        expr: Box<Span<Self>>,
    },

    Neg {
        minus_token: Token<'a>,
        expr: Box<Span<Self>>,
    },

    Add {
        lhs: Box<Span<Self>>,
        plus_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Sub {
        lhs: Box<Span<Self>>,
        minus_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Mul {
        lhs: Box<Span<Self>>,
        star_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Div {
        lhs: Box<Span<Self>>,
        slash_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Eq {
        lhs: Box<Span<Self>>,
        eqeq_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Ne {
        lhs: Box<Span<Self>>,
        ne_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Gt {
        lhs: Box<Span<Self>>,
        gt_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Ge {
        lhs: Box<Span<Self>>,
        ge_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Lt {
        lhs: Box<Span<Self>>,
        lt_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    Le {
        lhs: Box<Span<Self>>,
        le_token: Token<'a>,
        rhs: Box<Span<Self>>,
    },

    #[default]
    None,
}
