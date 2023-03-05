use std::fmt::Debug;

use crate::token::Token;
use plex::lexer;

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"\s"# => Token::Whitespace,
    r#"[0-9]+"# => Token::Integer(text.parse().unwrap()),
    r#""[^"]*""# => Token::String(text[1..text.len()-1].to_owned()),

    r#"\("# => Token::LeftParen,
    r#"\)"# => Token::RightParen,
    r#"\["# => Token::LeftBracket,
    r#"\]"# => Token::RightBracket,
    r#"\{"# => Token::LeftBrace,
    r#"\}"# => Token::RightBrace,
    r#"\+"# => Token::Plus,
    r#"\+="# => Token::PlusEq,
    r#"-"# => Token::Minus,
    r#"-="# => Token::MinusEq,
    r#"\*"# => Token::Star,
    r#"\*="# => Token::StarEq,
    r#"\/"# => Token::Slash,
    r#"\/="# => Token::SlashEq,
    r#"%"# => Token::Percent,
    r#"%="# => Token::PercentEq,
    r#"="# => Token::Eq,
    r#"=="# => Token::EqEq,
    r#"!="# => Token::Ne,
    r#">"# => Token::Gt,
    r#">="# => Token::Ge,
    r#"<"# => Token::Lt,
    r#"<="# => Token::Le,
    r#"\|"# => Token::Or,
    r#"\|="# => Token::OrEq,
    r#"\&"# => Token::And,
    r#"\&="# => Token::AndEq,
    r#"\^"# => Token::Xor,
    r#"\^="# => Token::XorEq,
    r#"<<"# => Token::Shl,
    r#"<<="# => Token::ShlEq,
    r#">>"# => Token::Shr,
    r#">>="# => Token::ShrEq,
    r#"\?"# => Token::Question,
    r#"!"# => Token::Bang,
    r#"_"# => Token::Underscore,
    r#"\~"# => Token::Not,
    r#"\."# => Token::Dot,
    r#","# => Token::Comma,
    r#";"# => Token::Semi,
    r#":"# => Token::Colon,
    r#"let"# => Token::Let,
    r#"if"# => Token::If,
    r#"else"# => Token::Else,
    r#"drop"# => Token::Drop,
    r#"while"# => Token::While,
    r#"for"# => Token::For,
    r#"break"# => Token::Break,
    r#"continue"# => Token::Continue,
    r#"fn"# => Token::Fn,
    r#"print"# => Token::Print,
    r#"typeof"# => Token::Typeof,
    r#"true"# => Token::True,
    r#"false"# => Token::False,

    r#"."# => panic!("unexpected '{text}'")

}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span(pub usize, pub usize);

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span(lo, hi))
            } else {
                return None;
            };
            match tok {
                // Token::Whitespace | Token::Comment => {
                //     continue;
                // }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
