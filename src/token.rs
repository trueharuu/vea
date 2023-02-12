#[derive(Debug, Clone)]
pub enum Token {
    Ident(String), // abc

    Typeof, // typeof a
    Print, // print a
    Env, // env a
    Fn, // fn

    Integer(i64), // 123
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