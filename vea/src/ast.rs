use crate::common::VeaErr;
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

    Return {
        return_token: Span<Token<'a>>,
        semi_token: Span<Token<'a>>,

        value: Box<Span<Self>>,
    },

    Literal {
        value: Literal<'a>,
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

    While {
        while_token: Span<Token<'a>>,
        condition: Box<Span<Self>>,
        then: Box<Span<Self>>,
    },

    FnDecl {
        fn_token: Span<Token<'a>>,
        name: Span<&'a str>,
        left_paren: Span<Token<'a>>,
        arguments: Vec<Span<&'a str>>,
        right_paren: Span<Token<'a>>,
        block: Box<Span<Self>>
    },

    FnCall {
        access: Box<Span<Self>>,
        left_paren: Span<Token<'a>>,
        arguments: Vec<Span<Self>>,
        right_paren: Span<Token<'a>>,
    },

    Let {
        let_token: Span<Token<'a>>,
        ident: Span<&'a str>,
        eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    Assign {
        ident: Span<&'a str>,
        eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    AddAssign {
        ident: Span<&'a str>,
        plus_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    SubAssign {
        ident: Span<&'a str>,
        minus_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    MulAssign {
        ident: Span<&'a str>,
        star_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    DivAssign {
        ident: Span<&'a str>,
        slash_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    RemAssign {
        ident: Span<&'a str>,
        percent_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    ShlAssign {
        ident: Span<&'a str>,
        shl_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    ShrAssign {
        ident: Span<&'a str>,
        shr_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    AndAssign {
        ident: Span<&'a str>,
        and_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    OrAssign {
        ident: Span<&'a str>,
        pipe_eq_token: Span<Token<'a>>,
        expr: Box<Span<Self>>,
        semi_token: Span<Token<'a>>,
    },

    XorAssign {
        ident: Span<&'a str>,
        caret_eq_token: Span<Token<'a>>,
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

    Rem {
        lhs: Box<Span<Self>>,
        percent_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Shl {
        lhs: Box<Span<Self>>,
        shl_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Shr {
        lhs: Box<Span<Self>>,
        shr_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    And {
        lhs: Box<Span<Self>>,
        and_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Or {
        lhs: Box<Span<Self>>,
        pipe_token: Span<Token<'a>>,
        rhs: Box<Span<Self>>,
    },

    Xor {
        lhs: Box<Span<Self>>,
        caret_token: Span<Token<'a>>,
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

    Error(VeaErr),

    #[default]
    None,
}
