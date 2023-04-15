use chumsky::prelude::*;
use chumsky::primitive::select;

use crate::ast::Expr;
use crate::ast::Literal;
use crate::choice;
use crate::common::Rebox;
use crate::common::Span;
use crate::common::Spanned;
use crate::common::Tag;
use crate::lexer::Token;

type ParserInput<'t, 's> = chumsky::input::SpannedInput<Token<'s>, Span, &'t [Spanned<Token<'s>>]>;
pub fn parser<'t, 's: 't>() -> impl Parser<
    't,
    ParserInput<'t, 's>,
    Vec<Spanned<Expr<'s>>>,
    chumsky::extra::Err<Rich<'t, Token<'s>, Span>>,
> + Clone {
    let ident = select(move |x, _| match x {
        Token::Ident(z) => Some(z),
        _ => None,
    })
    .labelled("ident");

    let expr: _ = recursive(|expr: _| {
        let atom = select(move |x, s| match x {
            Token::Ident(z) => Some(Expr::Variable(z)),
            Token::True => Some(Expr::Literal(Literal::Boolean(true).tag(s))),
            Token::False => Some(Expr::Literal(Literal::Boolean(true).tag(s))),
            Token::String(z) => Some(Expr::Literal(Literal::String(z.to_owned()).tag(s))),
            Token::Number(i) => Some(Expr::Literal(Literal::Number(i).tag(s))),
            _ => None,
        })
        .or(just(Token::LeftParen)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::RightParen)))
        .boxed()
        .map_with_span(|x, s| (x, s));

        let unary: _ = choice! {
            just(Token::Bang)
                .ignore_then(atom.clone())
                .map(|x| Expr::Not(x.rebox())),
            just(Token::Minus)
                .ignore_then(atom.clone())
                .map(|x| Expr::Neg(x.rebox()))
        };

        let sum: _ = atom
            .clone()
            .then_ignore(just(Token::Plus))
            .then(atom.clone())
            .map(|(x, y)| Expr::Add(x.rebox(), y.rebox()))
            .or(atom
                .clone()
                .then_ignore(just(Token::Minus))
                .then(atom.clone())
                .map(|(x, y)| Expr::Sub(x.rebox(), y.rebox())))
            .boxed();

        let product: _ = atom
            .clone()
            .then_ignore(just(Token::Star))
            .then(atom.clone())
            .map(|(x, y)| Expr::Mul(x.rebox(), y.rebox()))
            .or(atom
                .clone()
                .then_ignore(just(Token::Slash))
                .then(atom.clone())
                .map(|(x, y)| Expr::Div(x.rebox(), y.rebox())))
            .boxed();

        let cmp: _ = choice! {
            atom.clone()
                .then_ignore(just(Token::Gt))
                .then(atom.clone())
                .map(|(x, y)| Expr::Gt(x.rebox(), y.rebox())),
            atom.clone()
                .then_ignore(just(Token::Ge))
                .then(atom.clone())
                .map(|(x, y)| Expr::Ge(x.rebox(), y.rebox())),
            atom.clone()
                .then_ignore(just(Token::Lt))
                .then(atom.clone())
                .map(|(x, y)| Expr::Lt(x.rebox(), y.rebox())),
            atom.clone()
                .then_ignore(just(Token::Le))
                .then(atom.clone())
                .map(|(x, y)| Expr::Le(x.rebox(), y.rebox()))
        };

        let eq: _ = choice! {
            atom.clone()
                .then_ignore(just(Token::EqEq))
                .then(atom.clone())
                .map(|(x, y)| Expr::Eq(x.rebox(), y.rebox())),
            atom.clone()
                .then_ignore(just(Token::Ne))
                .then(atom.clone())
                .map(|(x, y)| Expr::Ne(x.rebox(), y.rebox()))

        };

        let items: _ = atom
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>();

        let array: _ = items
            .clone()
            .map(Expr::List)
            .delimited_by(just(Token::LeftBracket), just(Token::RightBracket));

        let set: _ = items
            .clone()
            .map(Expr::Set)
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

        choice!(
            array,
            set,
            eq,
            cmp,
            product,
            sum,
            unary,
            atom.map(|(x, _)| x)
        )
    })
    .boxed();

    let kwlet: _ = just(Token::Let)
        .ignore_then(ident)
        .then_ignore(just(Token::Eq))
        .then(expr.clone())
        .then_ignore(just(Token::Semi))
        .map_with_span(|(name, value), s| Expr::Let(name, value.tag(s).rebox()));

    let kwprint: _ = just(Token::Print)
        .ignore_then(expr.clone())
        .then_ignore(just(Token::Semi))
        .map_with_span(|x, s| Expr::Print(x.tag(s).rebox()));

    let stmt: _ = choice((kwlet, kwprint))
        .map_with_span(|x, y| (x, y))
        .repeated()
        .collect();

    stmt
}
