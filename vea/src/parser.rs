use chumsky::prelude::*;
use chumsky::primitive::select;

use crate::ast::Expr;

use crate::choice;
use crate::common::Tag;
use crate::literal::Literal;
use crate::span::RawSpan;
use crate::span::Span;

use crate::lexer::Token;

#[must_use]
pub fn parser<'t, 's: 't>() -> impl Parser<
    't,
    &'t [Token<'s>],
    Vec<Span<Expr<'s>>>,
    chumsky::extra::Err<Rich<'t, Token<'s>, RawSpan>>,
> + Clone {
    let ident = select(move |f, s| match f {
        Token::Ident(t) => Some(Span(t, s)),
        _ => None,
    });

    let stmt = recursive(|sel| {
        let block = just(Token::LeftBrace)
            .map_with_span(Span)
            .then(sel.clone().repeated().collect::<Vec<_>>())
            .then(just(Token::RightBrace).map_with_span(Span))
            .map_with_span(|((left_brace, exprs), right_brace), s| {
                Expr::Block {
                    left_brace,
                    right_brace,
                    exprs,
                }
                .t(s)
            })
            .boxed();

        let kwfn = group((
            just(Token::Fn).map_with_span(Span),
            ident,
            just(Token::LeftParen).map_with_span(Span),
            ident.separated_by(just(Token::Comma)).collect(),
            just(Token::RightParen).map_with_span(Span),
            block.clone(),
        ))
        .map_with_span(
            |(fn_token, name, left_paren, arguments, right_paren, block), s| {
                Expr::FnDecl {
                    fn_token,
                    name,
                    left_paren,
                    arguments,
                    right_paren,
                    block: Box::new(block),
                }
                .t(s)
            },
        )
        .boxed();

        macro_rules! kwlet {
            ($s:ident) => {
                just(Token::Let)
                    .map_with_span(Span)
                    .then(ident)
                    .then(just(Token::Eq).map_with_span(Span))
                    .then($s.clone())
                    .then(just(Token::Semi).map_with_span(Span))
                    .map_with_span(|((((let_token, ident), eq_token), expr), semi_token), s| {
                        Expr::Let {
                            eq_token,
                            expr,
                            ident,
                            let_token,
                            semi_token,
                        }
                        .t(s)
                    })
            };
        }

        let expr = recursive(|eel| {
            let kgroup = group((
                just(Token::LeftParen).map_with_span(Span),
                eel.clone(),
                just(Token::RightParen).map_with_span(Span),
            ))
            .map_with_span(|(left_paren, expr, right_paren), s| {
                Expr::Group {
                    left_paren,
                    right_paren,
                    expr,
                }
                .t(s)
            })
            .boxed();

            // aaa
            let idx = group((
                eel.clone(),
                just(Token::LeftBracket),
                eel.clone(),
                just(Token::RightBracket),
            ))
            .map_with_span(|(parent, _, child, _), s| Expr::Chain { parent, child }.t(s));

            let atom = select(move |f, s| match f {
                Token::Ident(t) => Some(Expr::Access { ident: t.t(s) }),
                Token::Number(t) => Some(Expr::Literal {
                    value: Literal::Integer(t),
                }),
                Token::Underscore => Some(Expr::Literal {
                    value: Literal::None,
                }),
                Token::String(t) => Some(Expr::Literal {
                    value: Literal::String(t),
                }),
                Token::True => Some(Expr::Literal {
                    value: Literal::Bool(true),
                }),
                Token::False => Some(Expr::Literal {
                    value: Literal::Bool(false),
                }),

                _ => None,
            })
            .map_with_span(Span)
            .or(idx.clone())
            .or(kgroup)
            .boxed();

            let key_or_fn = kwfn.clone().or(kwlet!(eel)).boxed();

            let obj = group((
                just(Token::Struct).map_with_span(Span),
                just(Token::LeftBrace).map_with_span(Span),
                key_or_fn.repeated().collect::<Vec<_>>(),
                just(Token::RightBrace).map_with_span(Span),
            ))
            .map_with_span(|(struct_token, left_brace, exprs, right_brace), s| {
                Expr::Object {
                    struct_token,
                    left_brace,
                    exprs,
                    right_brace,
                }
                .t(s)
            })
            .boxed();

            let set = group((
                just(Token::Set).map_with_span(Span),
                just(Token::LeftBrace).map_with_span(Span),
                eel.clone()
                    .separated_by(just(Token::Comma))
                    .collect::<Vec<_>>(),
                just(Token::RightBrace).map_with_span(Span),
            ))
            .map_with_span(|(set_token, left_brace, exprs, right_brace), s| {
                Expr::Set {
                    set_token,
                    left_brace,
                    exprs: exprs.into_iter().map(|x| *x).collect(),
                    right_brace,
                }
                .t(s)
            })
            .boxed();

            macro_rules! op {
                (1 $token:path, $expr:ident, $op:ident, $e: expr) => {
                    group((just($token).map_with_span(Span), atom.clone()))
                        .map(|($op, $expr)| ($op, Box::new($expr)))
                        .map_with_span(|($op, $expr), s| $e.t(s))
                        .boxed()
                };
                (2 $token:path, $lhs:ident $op:ident $rhs:ident, $e:expr) => {
                    group((atom.clone(), just($token).map_with_span(Span), atom.clone()))
                        .map(|($lhs, $op, $rhs)| (Box::new($lhs), $op, Box::new($rhs)))
                        .map_with_span(|($lhs, $op, $rhs), s| $e.t(s))
                        .boxed()
                };
            }

            let unary = choice![
                op![1 Token::Bang, expr, bang_token, Expr::Not { bang_token, expr }],
                op![1 Token::Minus, expr, minus_token, Expr::Neg { minus_token, expr }],
            ]
            .boxed();

            let sum = choice![
                op![2 Token::Plus, lhs plus_token rhs, Expr::Add { lhs, plus_token, rhs }],
                op![2 Token::Minus, lhs minus_token rhs, Expr::Sub { lhs, minus_token, rhs }],
            ]
            .boxed();

            let product = choice![
                op![2 Token::Star, lhs star_token rhs, Expr::Mul { lhs, star_token, rhs }],
                op![2 Token::Slash, lhs slash_token rhs, Expr::Div { lhs, slash_token, rhs }],
                op![2 Token::Percent, lhs percent_token rhs, Expr::Rem { lhs, percent_token, rhs }],
            ]
            .boxed();

            let cmp = choice![
                op![2 Token::EqEq, lhs eqeq_token rhs, Expr::Eq { lhs, eqeq_token, rhs }],
                op![2 Token::Ne, lhs ne_token rhs, Expr::Ne { lhs, ne_token, rhs }],
                op![2 Token::Gt, lhs gt_token rhs, Expr::Gt { lhs, gt_token, rhs }],
                op![2 Token::Ge, lhs ge_token rhs, Expr::Ge { lhs, ge_token, rhs }],
                op![2 Token::Lt, lhs lt_token rhs, Expr::Lt { lhs, lt_token, rhs }],
                op![2 Token::Le, lhs le_token rhs, Expr::Le { lhs, le_token, rhs }],
            ]
            .boxed();

            let bitwise = choice![
                op![2 Token::And, lhs and_token rhs, Expr::And { lhs, and_token, rhs }],
                op![2 Token::Caret, lhs caret_token rhs, Expr::Xor { lhs, caret_token, rhs }],
                op![2 Token::Pipe, lhs pipe_token rhs, Expr::Or { lhs, pipe_token, rhs }],
            ]
            .boxed();

            let fncall = group((
                ident.map_with_span(|ident, s| Box::new(Expr::Access { ident }.t(s))),
                just(Token::LeftParen).map_with_span(Span),
                eel.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .map(|x| x.iter().map(|x| *x.clone()).collect::<Vec<_>>()),
                just(Token::RightParen).map_with_span(Span),
            ))
            .map_with_span(|(access, left_paren, arguments, right_paren), s| {
                Expr::FnCall {
                    access,
                    arguments,
                    left_paren,
                    right_paren,
                }
                .t(s)
            })
            .boxed();

            choice![fncall, obj, set, bitwise, cmp, product, sum, unary, atom].map(Box::new)
        });

        let kwlet = kwlet!(expr);
        let assign = {
            macro_rules! assign_op {
                ($t:path, $ident:ident $name:ident $expr:ident $semi_token:ident, $e:expr) => {
                    group((
                        ident,
                        just($t).map_with_span(Span),
                        expr.clone(),
                        just(Token::Semi).map_with_span(Span),
                    ))
                    .map_with_span(|($ident, $name, $expr, $semi_token), s| $e.t(s))
                };
                (* $(: $t:path, $ident:ident $name:ident $expr:ident $semi_token:ident, $e:expr)*) => {
                    $(assign_op![$t, $ident $name $expr $semi_token, $e], )*
                }
            }

            choice![
                assign_op![
                    Token::AndEq,
                    ident and_eq_token expr semi_token,
                    Expr::AndAssign {
                        ident,
                        and_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::CaretEq,
                    ident caret_eq_token expr semi_token,
                    Expr::XorAssign {
                        ident,
                        caret_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::Eq,
                    ident eq_token expr semi_token,
                    Expr::Assign {
                        ident,
                        eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::MinusEq,
                    ident minus_eq_token expr semi_token,
                    Expr::SubAssign {
                        ident,
                        minus_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::PercentEq,
                    ident percent_eq_token expr semi_token,
                    Expr::RemAssign {
                        ident,
                        percent_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::PipeEq,
                    ident pipe_eq_token expr semi_token,
                    Expr::OrAssign {
                        ident,
                        pipe_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::PlusEq,
                    ident plus_eq_token expr semi_token,
                    Expr::AddAssign {
                        ident,
                        plus_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::ShlEq,
                    ident shl_eq_token expr semi_token,
                    Expr::ShlAssign {
                        ident,
                        shl_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::ShrEq,
                    ident shr_eq_token expr semi_token,
                    Expr::ShrAssign {
                        ident,
                        shr_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::SlashEq,
                    ident slash_eq_token expr semi_token,
                    Expr::DivAssign {
                        ident,
                        slash_eq_token,
                        expr,
                        semi_token
                    }
                ],
                assign_op![
                    Token::StarEq,
                    ident star_eq_token expr semi_token,
                    Expr::MulAssign {
                        ident,
                        star_eq_token,
                        expr,
                        semi_token
                    }
                ],
            ]
        };
        let kwreturn = chumsky::prelude::group((
            just(Token::Return).map_with_span(Span),
            expr,
            just(Token::Semi).map_with_span(Span),
        ))
        .map_with_span(|(return_token, value, semi_token), s| {
            Expr::Return {
                return_token,
                semi_token,
                value,
            }
            .t(s)
        });

        choice![block, kwfn, kwreturn, kwlet, assign]
    });

    stmt.repeated().at_least(1).collect()
}
