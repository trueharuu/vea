use std::fmt::Display;

use chumsky::prelude::*;
use dbg_pls::DebugPls;

use crate::choice;
use crate::common::Spanned;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, DebugPls)]
pub enum Token<'a> {
    Ident(&'a str),  // abc
    Number(i64),     // 123
    String(&'a str), // "abc"

    Let,   // let
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
) -> impl Parser<'s, &'s str, Vec<Spanned<Token<'s>>>, chumsky::extra::Err<chumsky::error::Rich<'s, char>>>
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

    let string: _ = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map_slice(Token::String)
        .boxed()
        .labelled("string");

    let op: _ = choice! {
        just("!=").to(Token::Ne),
        just("==").to(Token::EqEq),
        just(">=").to(Token::Ge),
        just("<=").to(Token::Le),
        just('+').to(Token::Plus),
        just('-').to(Token::Minus),
        just('*').to(Token::Star),
        just('/').to(Token::Slash),
        just('=').to(Token::Eq),
        just('!').to(Token::Bang),
        just('_').to(Token::Underscore),
        just('>').to(Token::Gt),
        just('<').to(Token::Lt),
        just('~').to(Token::Tilde),
        just('|').to(Token::Pipe),
        just('&').to(Token::And),
        just('^').to(Token::Caret),
        just('?').to(Token::Question),
        just('%').to(Token::Percent)
    }
    .boxed()
    .labelled("operator");

    let ctrl: _ = just('{')
        .to(Token::LeftBrace)
        .or(just('}').to(Token::RightBrace))
        .or(just('[').to(Token::LeftBracket))
        .or(just(']').to(Token::RightBracket))
        .or(just('(').to(Token::LeftParen))
        .or(just(')').to(Token::RightParen))
        .or(just(',').to(Token::Comma))
        .or(just(';').to(Token::Semi))
        .boxed();
    // .labelled("control");

    let kw: _ = just("let")
        .to(Token::Let)
        .or(just("true").to(Token::True))
        .or(just("false").to(Token::False))
        .or(just("print").to(Token::Print))
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
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
