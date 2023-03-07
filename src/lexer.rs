use std::fmt::Debug;

use crate::token::Token;
use plex::lexer;

lexer! {
    fn next_token(text: 'a) -> Result<Token, String>;


    r#"\("# => Ok(Token::LeftParen),
    r#"\)"# => Ok(Token::RightParen),
    r#"\["# => Ok(Token::LeftBracket),
    r#"\]"# => Ok(Token::RightBracket),
    r#"\{"# => Ok(Token::LeftBrace),
    r#"\}"# => Ok(Token::RightBrace),
    r#"\+"# => Ok(Token::Plus),
    r#"\+="# => Ok(Token::PlusEq),
    r#"-"# => Ok(Token::Minus),
    r#"-="# => Ok(Token::MinusEq),
    r#"\*"# => Ok(Token::Star),
    r#"\*="# => Ok(Token::StarEq),
    r#"\/"# => Ok(Token::Slash),
    r#"\/="# => Ok(Token::SlashEq),
    r#"%"# => Ok(Token::Percent),
    r#"%="# => Ok(Token::PercentEq),
    r#"="# => Ok(Token::Eq),
    r#"=="# => Ok(Token::EqEq),
    r#"!="# => Ok(Token::Ne),
    r#">"# => Ok(Token::Gt),
    r#">="# => Ok(Token::Ge),
    r#"<"# => Ok(Token::Lt),
    r#"<="# => Ok(Token::Le),
    r#"\|"# => Ok(Token::Or),
    r#"\|="# => Ok(Token::OrEq),
    r#"\&"# => Ok(Token::And),
    r#"\&="# => Ok(Token::AndEq),
    r#"\^"# => Ok(Token::Xor),
    r#"\^="# => Ok(Token::XorEq),
    r#"<<"# => Ok(Token::Shl),
    r#"<<="# => Ok(Token::ShlEq),
    r#">>"# => Ok(Token::Shr),
    r#">>="# => Ok(Token::ShrEq),
    r#"\?"# => Ok(Token::Question),
    r#"!"# => Ok(Token::Bang),
    r#"_"# => Ok(Token::Underscore),
    r#"\~"# => Ok(Token::Not),
    r#"\."# => Ok(Token::Dot),
    r#","# => Ok(Token::Comma),
    r#";"# => Ok(Token::Semi),
    r#":"# => Ok(Token::Colon),
    r#"let"# => Ok(Token::Let),
    r#"if"# => Ok(Token::If),
    r#"else"# => Ok(Token::Else),
    r#"drop"# => Ok(Token::Drop),
    r#"while"# => Ok(Token::While),
    r#"for"# => Ok(Token::For),
    r#"break"# => Ok(Token::Break),
    r#"continue"# => Ok(Token::Continue),
    r#"fn"# => Ok(Token::Fn),
    r#"print"# => Ok(Token::Print),
    r#"typeof"# => Ok(Token::Typeof),
    r#"true"# => Ok(Token::True),
    r#"false"# => Ok(Token::False),

    r#"[a-z_]+"# => Ok(Token::Ident(text.to_string())),
    r#"[ \t\n\r]"# => Ok(Token::Whitespace),
    r#"[0-9]+"# => Ok(Token::Integer(text.parse().unwrap())),
    r#""[^"]*""# => Ok(Token::String(text[1..text.len()-1].to_owned())),

    r#"."# => Err(format!("Unexpected '{text}'"))

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
    type Item = (Result<Token, String>, Span);
    fn next(&mut self) -> Option<(Result<Token, String>, Span)> {
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
                Ok(Token::Whitespace | Token::Comment) => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
