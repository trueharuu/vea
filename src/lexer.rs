use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String), // abc

    Typeof, // typeof a
    Print, // print a
    Env, // env a
    Fn, // fn

    Integer(Integer), // 123
    String(String), // "abc"
    True, // true
    False, // false

    Equals, // =
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Bang, // !
    LeftParen, // (
    RightParen, // )
    LeftBracket, // [
    RightBracket, // ]
    LeftBrace,
    RightBrace,
    Comma, // ,
    Dot, // .
    Semi, // ;
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=
    Eq, // ==
    Ne, // !=

    Whitespace,
    Comment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Integer {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r\n]+"# => Token::Whitespace,
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Token::Comment,

    r#"print"# => Token::Print,
    r#"typeof"# => Token::Typeof,

    r#"[0-9]+([ui](8|16|32|64|128|size))?"# => {
      let mut parts = text.split_inclusive(&['i', 'u']);

      let [mut value, ty] = parts.next_chunk().unwrap();
      let typ = if value.ends_with(&['i', 'u']) {
          let last = &value[value.len() - 1..];
          value = &value[..value.len() - 1];
          vec![last.to_owned(), ty.to_owned()].concat()
      } else {
        ty.to_owned()
      };
  
      match typ.as_str() {
        "i8" => Token::Integer(Integer::I8(value.parse::<i8>().unwrap())),
        "i16" => Token::Integer(Integer::I16(value.parse::<i16>().unwrap())),
        "i32" => Token::Integer(Integer::I32(value.parse::<i32>().unwrap())),
        "i64" => Token::Integer(Integer::I64(value.parse::<i64>().unwrap())),
        "i128" => Token::Integer(Integer::I128(value.parse::<i128>().unwrap())),
        "isize" => Token::Integer(Integer::ISize(value.parse::<isize>().unwrap())),
        "u8" => Token::Integer(Integer::U8(value.parse::<u8>().unwrap())),
        "u16" => Token::Integer(Integer::U16(value.parse::<u16>().unwrap())),
        "u32" => Token::Integer(Integer::U32(value.parse::<u32>().unwrap())),
        "u64" => Token::Integer(Integer::U64(value.parse::<u64>().unwrap())),
        "u128" => Token::Integer(Integer::U128(value.parse::<u128>().unwrap())),
        "usize" => Token::Integer(Integer::USize(value.parse::<usize>().unwrap())),
        &_ => panic!("unknown integer type {ty}"),
      }
    }

    r#"\["# => Token::LeftBracket,
    r#"\]"# => Token::RightBracket,

    r#"\{"# => Token::LeftBrace,
    r#"\}"# => Token::RightBrace,

    r#"true"# => Token::True,
    r#"false"# => Token::False,

    r#","# => Token::Comma,
    
    r#"\".*\""# => 
        Token::String(text[1..(text.len() - 1)].to_owned()),
    r#"env"# => Token::Env,

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),

    r#"="# => Token::Equals,
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Star,
    r#"/"# => Token::Slash,
    r#"\("# => Token::LeftParen,
    r#"\)"# => Token::RightParen,
    r#";"# => Token::Semi,
    r#"!"# => Token::Bang,
    
    r#">"# => Token::Gt,
    r#"<"# => Token::Lt,
    
    r#">="# => Token::Ge,
    r#"<="# => Token::Le,
    
    r#"=="# => Token::Eq,
    r#"!="# => Token::Ne,

    r#"\."# => Token::Dot,

    r#"."# => panic!("unexpected character: {}", text),
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

#[derive(Debug, Clone, Copy)]
pub struct Span(pub usize, pub usize);

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
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}