use chumsky::error::Rich;
use chumsky::prelude::*;

use crate::choice;
use crate::common::VeaErr;
use crate::span::Span;
// use crate::void;
// use crate::special_chars;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Token<'a> {
    Ident(&'a str), // abc
    Number(i64),    // 123
    String(String), // 'abc'

    Let,    // let
    If,     // if
    Else,   // else
    Print,  // print
    True,   // true
    False,  // false
    While,  // while
    For,    // for
    Fn,     // fn
    Return, // return

    Quote, // '

    Plus,      // +
    PlusEq,    // +=
    Minus,     // -
    MinusEq,   // -=
    Star,      // *
    StarEq,    // *=
    Slash,     // /
    SlashEq,   // /=
    Pipe,      // |
    PipeEq,    // |=
    And,       // &
    AndEq,     // &=
    Caret,     // ^
    CaretEq,   // ^=
    Percent,   // %
    PercentEq, // %=
    Shl,       // <<
    ShlEq,     // <<=
    Shr,       // >>
    ShrEq,     // >>=

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
    Question,   // ?

    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    Semi,         // ;

    Error(VeaErr),
}

// impl<'a> Display for Token<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Self::And => "&",
//                 _ => "?",
//             }
//         )
//     }
// }

pub fn lexer<'s>(
) -> impl Parser<'s, &'s str, Vec<Span<Token<'s>>>, chumsky::extra::Err<Rich<'s, char>>> {
    let num: _ = text::int(10)
        .from_str()
        .map(|x| x.map_or(Token::Error(VeaErr::IntegerOverflow), Token::Number))
        .labelled("integer");

    let ident: _ = one_of("abcdefghijklmnopqrstuvwxyz_")
        .repeated()
        .at_least(1)
        .map_slice(Token::Ident)
        .boxed()
        .labelled("ident");

    let quote_char = just('\'').to(Token::Quote);

    let string: _ = quote_char
        .clone()
        .ignore_then(none_of('\'').repeated())
        .then_ignore(quote_char)
        .map_slice(|x: &str| Token::String(x.to_owned()))
        .boxed()
        .labelled("string");

    let op: _ = choice! {
        just("&=").to(Token::AndEq),
        just("!=").to(Token::Ne),
        just("==").to(Token::EqEq),
        just(">=").to(Token::Ge),
        just("<=").to(Token::Le),
        just("^=").to(Token::CaretEq),
        just("+=").to(Token::PlusEq),
        just("%=").to(Token::PercentEq),
        just("-=").to(Token::MinusEq),
        just("<<=").to(Token::ShlEq),
        just(">>=").to(Token::ShrEq),
        just("*=").to(Token::StarEq),
        just(">>").to(Token::Shr),
        just("/=").to(Token::SlashEq),
        just("<<").to(Token::Shl),
        just("|=").to(Token::PipeEq),
        just('+').to(Token::Plus),
        just('-').to(Token::Minus),
        just("*").to(Token::Star),
        just("/").to(Token::Slash),
        just('=').to(Token::Eq),
        just('!').to(Token::Bang),
        just('_').to(Token::Underscore),
        just('>').to(Token::Gt),
        just('<').to(Token::Lt),
        just("~").to(Token::Tilde),
        just("|").to(Token::Pipe),
        just("&").to(Token::And),
        just("^").to(Token::Caret),
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
    .labelled("control character");

    let kw: _ = choice! {
        just("let").to(Token::Let),
        just("if").to(Token::If),
        just("else").to(Token::Else),
        just("true").to(Token::True),
        just("false").to(Token::False),
        just("print").to(Token::Print),
        just("while").to(Token::While),
        just("for").to(Token::For),
        just("fn").to(Token::Fn),
        just("return").to(Token::Return),
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
        .or(string)
        .or(ident)
        .or(op)
        .or(ctrl)
        // .or(quote_char)
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
