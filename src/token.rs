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