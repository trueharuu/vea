use chumsky::prelude::*;
use chumsky::primitive::select;

use crate::ast::Expr;

use crate::choice;
use crate::literal::Literal;
use crate::span::RawSpan;
use crate::span::Span;

use crate::common::{Rebox, Tag};
use crate::lexer::Token;

#[must_use]
pub fn parser<'t, 's: 't>() -> impl Parser<
    't,
    &'t [Token<'s>],
    Vec<Span<Expr<'s>>>,
    chumsky::extra::Err<Rich<'t, Token<'s>, RawSpan>>,
> + Clone {
    let total: _ = recursive(|expr| {
        let ident = select(|x, s| match x {
            Token::Ident(x) => Some(Span(x, s)),
            _ => None,
        });

        let inline = recursive(|z| {
            let atom = select(move |x, s| match x {
                Token::Ident(z) => Some(Expr::Access { ident: z.t(s) }),
                Token::True => Some(Expr::Literal {
                    value: Literal::Bool(true),
                }),
                Token::False => Some(Expr::Literal {
                    value: Literal::Bool(false),
                }),

                Token::Number(z) => Some(Expr::Literal {
                    value: Literal::Integer(z),
                }),

                Token::String(z) => Some(Expr::Literal {
                    value: Literal::String(z.to_owned()),
                }),

                _ => None,
            })
            .map_with_span(Span)
            .or(just(Token::LeftParen)
                .map_with_span(Span)
                .then(z.clone())
                .then(just(Token::RightParen).map_with_span(Span))
                .map_with_span(|((l, e), r): ((Span<Token>, Span<Expr>), Span<Token>), s| {
                    Expr::Group {
                        left_paren: l,
                        expr: e.rebox(),
                        right_paren: r,
                    }
                    .t(s)
                }))
            .boxed();

            let unary: _ = choice! {
                just(Token::Bang).map_with_span(Span)
                    .then(atom.clone())
                    .map_with_span(|(t, x), s| Expr::Not { bang_token: t, expr: x.rebox() }.t(s)),
                just(Token::Minus).map_with_span(Span)
                    .then(atom.clone())
                    .map_with_span(|(t, x), s| Expr::Neg { minus_token: t, expr: x.rebox() }.t(s))
            };

            let sum: _ = choice! {
                atom
                    .clone()
                    .then(just(Token::Plus).map_with_span(Span))
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
                    .then(just(Token::Minus).map_with_span(Span))
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
                    .then(just(Token::Star).map_with_span(Span))
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
                    .then(just(Token::Slash).map_with_span(Span))
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
                    .then(just(Token::Gt).map_with_span(Span))
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
                    .then(just(Token::Ge).map_with_span(Span))
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
                    .then(just(Token::Lt).map_with_span(Span))
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
                    .then(just(Token::Le).map_with_span(Span))
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
                    .then(just(Token::EqEq).map_with_span(Span))
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
                    .then(just(Token::Ne).map_with_span(Span))
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
            .map_with_span(Span)
            .then(ident)
            .then(just(Token::Eq).map_with_span(Span))
            .then(inline.clone())
            .then(just(Token::Semi).map_with_span(Span))
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

        let block = just(Token::LeftBrace)
            .map_with_span(Span)
            .then(expr.clone())
            .then(just(Token::RightBrace).map_with_span(Span))
            .map_with_span(|((lb, e), rb), s| {
                Expr::Block {
                    left_brace: lb,
                    right_brace: rb,
                    exprs: e,
                }
                .t(s)
            });

        let kwif = recursive(|kif| {
            just(Token::If)
                .map_with_span(Span)
                .then(
                    just(Token::LeftParen)
                        .map_with_span(Span)
                        .ignore_then(inline.clone())
                        .then_ignore(just(Token::RightParen).map_with_span(Span)),
                )
                .then(block.clone())
                .then(
                    just(Token::Else)
                        .map_with_span(Span)
                        .then(block.clone().or(kif).or_not())
                        .or_not(),
                )
                .map_with_span(|(((l, c), e), z), s| {
                    if let Some((p, q)) = z {
                        Expr::If {
                            if_token: l,
                            condition: c.rebox(),
                            then: e.rebox(),
                            else_token: Some(p),
                            other: q.map(Box::new),
                        }
                    } else {
                        Expr::If {
                            if_token: l,
                            condition: c.rebox(),
                            then: e.rebox(),
                            else_token: None,
                            other: None,
                        }
                    }
                    .t(s)
                })
        });

        let kwprint = just(Token::Print)
            .map_with_span(Span)
            .then(just(Token::LeftParen).map_with_span(Span))
            .then(inline.clone())
            .then(just(Token::RightParen).map_with_span(Span))
            .then(just(Token::Semi).map_with_span(Span))
            .map_with_span(|((((lp, p), e), rp), sm), s| {
                Expr::Print {
                    value: e.rebox(),
                    left_paren: lp,
                    print_token: p,
                    right_paren: rp,
                    semi_token: sm,
                }
                .t(s)
            });

        choice! { block, kwlet, kwif, kwprint, inline }
            .repeated()
            .collect()
    });

    total
}
