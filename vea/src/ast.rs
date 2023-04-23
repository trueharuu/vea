use crate::lexer::Token;
use crate::literal::Literal;
use crate::span::Span;

#[derive(Default, Clone, Debug)]
pub enum Expr<'a> {
    Access {
        ident: Span<&'a str>,
    },

    Print {
        left_paren: Span<Token<'a>>,
        print_token: Span<Token<'a>>,
        right_paren: Span<Token<'a>>,
        semi_token: Span<Token<'a>>,

        value: Box<Span<Self>>,
    },

    Literal {
        value: Literal,
    },

    Group {
        left_paren: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        right_paren: Span<Token<'a>>,
    },

    Block {
        left_brace: Span<Token<'a>>,
        exprs: Vec<Span<Self>>,
        right_brace: Span<Token<'a>>,
    },

    If {
        if_token: Span<Token<'a>>,
        condition: Box<Span<Self>>,
        then: Box<Span<Self>>,
        else_token: Option<Span<Token<'a>>>,
        other: Option<Box<Span<Self>>>,
    },

    Let {
        let_token: Span<Token<'a>>,
        ident: Span<&'a str>,
        eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    Not {
        bang_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
    },

    Neg {
        minus_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
    },

    Add {
        lhs: Box<Span<Self>>,
        plus_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Sub {
        lhs: Box<Span<Self>>,
        minus_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Mul {
        lhs: Box<Span<Self>>,
        star_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Div {
        lhs: Box<Span<Self>>,
        slash_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Eq {
        lhs: Box<Span<Self>>,
        eqeq_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Ne {
        lhs: Box<Span<Self>>,
        ne_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Gt {
        lhs: Box<Span<Self>>,
        gt_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Ge {
        lhs: Box<Span<Self>>,
        ge_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Lt {
        lhs: Box<Span<Self>>,
        lt_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Le {
        lhs: Box<Span<Self>>,
        le_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    #[default]
    None,
}
