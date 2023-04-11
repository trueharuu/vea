use chumsky::{error::Rich, extra::Err, prelude::*};

#[derive(Clone, Copy)]
pub enum Token<'a> {
    Ident(&'a str),
    Number(i64),
    String(&'a str),
    // keywords
    Let,
    // symbols
    Plus,
    Minus,
    // misc
    LeftBrace,
    RightBrace,
}

pub type Span = SimpleSpan<usize>;

pub fn lexer<'s>() -> impl Parser<'s, &'s str, Vec<(Token<'s>, Span)>> {
    let num: _ = text::int(10).from_str().unwrapped().map(Token::Number);
    let ident: _ = one_of("abcdefghijklmnopqrstuvwxyz_")
        .repeated()
        .map_slice(Token::Ident);
    let string: _ = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map_slice(Token::String);
    let op: _ = just('+').to(Token::Plus).or(just('-').to(Token::Minus));

    let comment: _ = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    let token: _ = num.or(ident).or(string).or(op);

    token
        .map_with_span(|tok, span: Span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
