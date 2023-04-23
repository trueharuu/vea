use std::fmt::Display;

use chumsky::prelude::*;

use crate::choice;
use crate::choice_just;
use crate::span::Span;
use crate::special_chars;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Token<'a> {
    Ident(&'a str),  // abc
    Number(i64),     // 123
    String(&'a str), // "abc"

    Let,   // let
    If,    // if
    Else,  // else
    Print, // print
    True,  // true
    False, // false

    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Eq,         // =
    Bang,       // !
    Underscore, // _
    EqEq,       // ==
    Ne,         // !=
    Gt,         // >
    Ge,         // >=
    Lt,         // <
    Le,         // <=
    Tilde,      // ~
    Pipe,       // |
    And,        // &
    Caret,      // ^
    Question,   // ?
    Percent,    // %

    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    Semi,         // ;
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "&",
                _ => "?",
            }
        )
    }
}

pub fn lexer<'s>(
) -> impl Parser<'s, &'s str, Vec<Span<Token<'s>>>, chumsky::extra::Err<chumsky::error::Rich<'s, char>>>
{
    let num: _ = text::int(10)
        .from_str()
        .unwrapped()
        .map(Token::Number)
        .labelled("integer");

    let ident: _ = one_of("abcdefghijklmnopqrstuvwxyz_")
        .repeated()
        .at_least(1)
        .map_slice(Token::Ident)
        .boxed()
        .labelled("ident");

    let string: _ = just('\'')
        .ignore_then(none_of('\'').repeated())
        .then_ignore(just('\''))
        .map_slice(Token::String)
        .boxed()
        .labelled("string");

    let op: _ = choice! {
        choice_just! { "!=", special_chars::NOT_EQUAL_TO }.to(Token::Ne),
        choice_just! { "==", special_chars::IDENTICAL_TO }.to(Token::EqEq),
        choice_just! { ">=", special_chars::GREATER_THAN_OR_EQUAL_TO }.to(Token::Ge),
        choice_just! { "<=", special_chars::LESS_THAN_OR_EQUAL_TO }.to(Token::Le),
        just('+').to(Token::Plus),
        just('-').to(Token::Minus),
        choice_just! { "*", special_chars::MULTIPLICATION_SIGN }.to(Token::Star),
        choice_just! { "/", special_chars::DIVISION_SIGN }.to(Token::Slash),
        just('=').to(Token::Eq),
        just('!').to(Token::Bang),
        just('_').to(Token::Underscore),
        just('>').to(Token::Gt),
        just('<').to(Token::Lt),
        choice_just! { "~", special_chars::NOT_SIGN }.to(Token::Tilde),
        choice_just! { "|", special_chars::LOGICAL_OR }.to(Token::Pipe),
        choice_just! { "&", special_chars::LOGICAL_AND }.to(Token::And),
        choice_just! { "^", special_chars::XOR }.to(Token::Caret),
        just('?').to(Token::Question),
        just('%').to(Token::Percent)
    }
    .boxed()
    .labelled("operator");

    let ctrl: _ = choice! {
        just('{').to(Token::LeftBrace),
        just('}').to(Token::RightBrace),
        just('[').to(Token::LeftBracket),
        just(']').to(Token::RightBracket),
        just('(').to(Token::LeftParen),
        just(')').to(Token::RightParen),
        just(',').to(Token::Comma),
        just(';').to(Token::Semi)
    }
    .boxed()
    .labelled("control");

    let kw: _ = choice! {
        just("let").to(Token::Let),
        just("if").to(Token::If),
        just("else").to(Token::Else),
        just("true").to(Token::True),
        just("false").to(Token::False),
        just("print").to(Token::Print)
    }
    .boxed()
    .labelled("keyword");

    let comment: _ = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .boxed()
        .labelled("comment");

    let token: _ = num
        .or(kw)
        .or(ident)
        .or(op)
        .or(ctrl)
        .or(string)
        .boxed()
        .labelled("token");

    token
        .map_with_span(Span)
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
