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
        })
        .boxed()
        .labelled("ident");

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
                    value: Literal::String(z),
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
            }
            .boxed();

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
            }
            .boxed();

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
                    }),

                atom
                    .clone()
                    .then(just(Token::Percent).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::Rem {
                            lhs: l.rebox(),
                            percent_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    })
            }
            .boxed();

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
            }
            .boxed();

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
            }
            .boxed();

            let bitwise: _ = choice! {
                atom
                    .clone()
                    .then(just(Token::Shl).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::Shl {
                            lhs: l.rebox(),
                            shl_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    }),
                atom
                    .clone()
                    .then(just(Token::Shr).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::Shr {
                            lhs: l.rebox(),
                            shr_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    }),
                atom
                    .clone()
                    .then(just(Token::And).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::And {
                            lhs: l.rebox(),
                            and_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    }),
                atom
                    .clone()
                    .then(just(Token::Pipe).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::Or {
                            lhs: l.rebox(),
                            pipe_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    }),
                atom
                    .clone()
                    .then(just(Token::Caret).map_with_span(Span))
                    .then(atom.clone())
                    .map_with_span(|((l, t), r), s| {
                        Expr::Xor {
                            lhs: l.rebox(),
                            caret_token: t,
                            rhs: r.rebox(),
                        }.t(s)
                    })
            }
            .boxed();

            let callee: _ = atom
                .clone()
                .then(just(Token::LeftParen).map_with_span(Span))
                .then(atom.clone().separated_by(just(Token::Comma)).collect())
                .then(just(Token::RightParen).map_with_span(Span))
                .map_with_span(|(((access, left_paren), arguments), right_paren), s| {
                    Expr::FnCall {
                        access: access.rebox(),
                        left_paren,
                        arguments,
                        right_paren,
                    }
                    .t(s)
                })
                .boxed();

            // precedence rocks!
            choice! { callee, eq, cmp, product, bitwise, sum, unary, atom }.boxed()
        });

        let kwlet: _ = just(Token::Let)
            .map_with_span(Span)
            .then(ident.clone())
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
            })
            .boxed();

        let assign_to = {
            macro_rules! make {
                (* $separator:path, $expr:path, $id:ident) => {
                    ident
                        .clone()
                        .then(just($separator).map_with_span(Span))
                        .then(inline.clone())
                        .then(just(Token::Semi).map_with_span(Span))
                        .map_with_span(|(((a, b), c), d), s| {
                            $expr {
                                ident: a,
                                $id: b,
                                expr: c.rebox(),
                                semi_token: d,
                            }
                            .t(s)
                        })
                        .boxed()
                };
            }

            let assignment: _ = make!(*Token::Eq, Expr::Assign, eq_token);
            let add_assign: _ = make!(*Token::PlusEq, Expr::AddAssign, plus_eq_token);
            let sub_assign: _ = make!(*Token::MinusEq, Expr::SubAssign, minus_eq_token);
            let mul_assign: _ = make!(*Token::StarEq, Expr::MulAssign, star_eq_token);
            let div_assign: _ = make!(*Token::SlashEq, Expr::DivAssign, slash_eq_token);
            let rem_assign: _ = make!(*Token::PercentEq, Expr::RemAssign, percent_eq_token);

            let shl_assign: _ = make!(*Token::ShlEq, Expr::AddAssign, plus_eq_token);
            let shr_assign: _ = make!(*Token::ShrEq, Expr::SubAssign, minus_eq_token);
            let and_assign: _ = make!(*Token::AndEq, Expr::MulAssign, star_eq_token);
            let oor_assign: _ = make!(*Token::PipeEq, Expr::DivAssign, slash_eq_token);
            let xor_assign: _ = make!(*Token::CaretEq, Expr::RemAssign, percent_eq_token);

            choice! { assignment, add_assign, sub_assign, mul_assign, div_assign, rem_assign, shl_assign, shr_assign, and_assign, oor_assign, xor_assign }.boxed()
        };

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
            })
            .boxed();

        let kwwhile = just(Token::While)
            .map_with_span(Span)
            .then(inline.clone())
            .then(block.clone())
            .map_with_span(|((while_token, c), t), s| {
                Expr::While {
                    while_token,
                    condition: c.rebox(),
                    then: t.rebox(),
                }
                .t(s)
            })
            .boxed();

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
        })
        .boxed();

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
            })
            .boxed();

        let kwreturn = just(Token::Return)
            .map_with_span(Span)
            .then(inline.clone())
            .then(just(Token::Semi).map_with_span(Span))
            .map_with_span(|((r, e), sm), s| {
                Expr::Return {
                    value: e.rebox(),
                    return_token: r,
                    semi_token: sm,
                }
                .t(s)
            })
            .boxed();

        let kwfn = just(Token::Fn)
            .map_with_span(Span)
            .then(ident.clone())
            .then(just(Token::LeftParen).map_with_span(Span))
            .then(ident.separated_by(just(Token::Comma)).collect())
            .then(just(Token::RightParen).map_with_span(Span))
            .then(block.clone())
            .map_with_span(
                |(((((fn_token, name), left_paren), arguments), right_paren), block), s| {
                    // () == x;
                    Expr::FnDecl {
                        fn_token,
                        name,
                        left_paren,
                        arguments,
                        right_paren,
                        block: block.rebox(),
                    }
                    .t(s)
                },
            )
            .boxed();

        let errs = select(move |x, _| match x {
            Token::Error(e) => Some(Expr::Error(e)),
            _ => None,
        })
        .map_with_span(Span);

        choice! { errs, block, kwlet, assign_to, kwfn, kwif, kwwhile, kwreturn, kwprint, inline }
            .boxed()
            .repeated()
            .collect()
    });

    total
}
