use chumsky::prelude::*;
use chumsky::primitive::select;

use crate::ast::Expr;

use crate::choice;
use crate::literal::Literal;
use crate::span::RawSpan;
use crate::span::Span;

use crate::common::{Rebox, Tag};
use crate::lexer::Token;

pub fn parser<'t, 's: 't>() -> impl Parser<
    't,
    &'t [Token<'s>],
    Vec<Span<Expr<'s>>>,
    chumsky::extra::Err<Rich<'t, Token<'s>, RawSpan>>,
> + Clone {
    let ident = select(|x, s| match x {
        Token::Ident(x) => Some(Span(x, s)),
        _ => None,
    });

    let expr = recursive(|expr| {
        let atom = select(move |x, s| match x {
            Token::Ident(z) => Some(Expr::Access { ident: z.t(s) }),
            Token::True => Some(Expr::Literal {
                value: Literal::Bool(true),
            }),
            Token::False => Some(Expr::Literal {
                value: Literal::Bool(false),
            }),

            _ => None,
        })
        .map_with_span(Span)
        .or(just(Token::LeftParen)
            .then(expr)
            .then(just(Token::RightParen))
            .map_with_span(|((l, e), r): ((Token, Span<Expr>), Token), s| {
                Expr::Group {
                    left_paren: l,
                    expr: e.rebox(),
                    right_paren: r,
                }
                .t(s)
            }))
        .boxed();

        let unary: _ = choice! {
            just(Token::Bang)
                .then(atom.clone())
                .map_with_span(|(t, x), s| Expr::Not { bang_token: t, expr: x.rebox() }.t(s)),
            just(Token::Minus)
                .then(atom.clone())
                .map_with_span(|(t, x), s| Expr::Neg { minus_token: t, expr: x.rebox() }.t(s))
        };

        let sum: _ = choice! {
            atom
                .clone()
                .then(just(Token::Plus))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Add {
                        lhs: l.rebox(),
                        plus_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Minus))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Sub {
                        lhs: l.rebox(),
                        minus_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                })
        };

        let product: _ = choice! {
            atom
                .clone()
                .then(just(Token::Star))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Mul {
                        lhs: l.rebox(),
                        star_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Slash))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Div {
                        lhs: l.rebox(),
                        slash_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                })
        };

        let cmp: _ = choice! {
            atom
                .clone()
                .then(just(Token::Gt))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Gt {
                        lhs: l.rebox(),
                        gt_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Ge))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Ge {
                        lhs: l.rebox(),
                        ge_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Lt))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Lt {
                        lhs: l.rebox(),
                        lt_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Le))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Le {
                        lhs: l.rebox(),
                        le_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                })
        };

        let eq: _ = choice! {
            atom
                .clone()
                .then(just(Token::Eq))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Eq {
                        lhs: l.rebox(),
                        eqeq_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                }),

            atom
                .clone()
                .then(just(Token::Ne))
                .then(atom.clone())
                .map_with_span(|((l, t), r), s| {
                    Expr::Ne {
                        lhs: l.rebox(),
                        ne_token: t,
                        rhs: r.rebox(),
                    }.t(s)
                })
        };

        // precedence rocks!
        choice! { eq, cmp, product, sum, unary, atom }
    });

    let kwlet: _ = just(Token::Let)
        .then(ident)
        .then(just(Token::Eq))
        .then(expr.clone())
        .then(just(Token::Semi))
        .map_with_span(|((((tl, i), e), te), ts), s| {
            Expr::Let {
                let_token: tl,
                ident: i,
                eq_token: e,
                expr: te.rebox(),
                semi_token: ts,
            }
            .t(s)
        });

    kwlet.repeated().collect()
}
