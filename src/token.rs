#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),    // abc
    Key(String),      // 1a
    Typeof,           // typeof a
    Print,            // print a
    Throw,            // throw a
    Env,              // env a
    Fn,               // fn
    Integer(Integer), // 123
    String(String),   // "abc"
    True,             // true
    False,            // false
    Equals,           // =
    Plus,             // +
    Minus,            // -
    Star,             // *
    Slash,            // /
    Bang,             // !
    Percent,          // %
    LeftParen,        // (
    RightParen,       // )
    LeftBracket,      // [
    RightBracket,     // ]
    LeftBrace,        // {
    RightBrace,       // }
    Comma,            // ,
    Dot,              // .
    Semi,             // ;
    Gt,               // >
    Ge,               // >=
    Lt,               // <
    Le,               // <=
    Eq,               // ==
    Ne,               // !=
    If,               // if
    Else,             // else
    While,            // while
    Whitespace,       //
    Comment,          // // hi
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
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

macro_rules! create_into {
    [$T:ident] => {
        impl From<Integer> for $T {
            fn from(value: Integer) -> $T {
                match value {
                    Integer::I8(i) => i as $T,
                    Integer::I16(i) => i as $T,
                    Integer::I32(i) => i as $T,
                    Integer::I64(i) => i as $T,
                    Integer::I128(i) => i as $T,
                    Integer::ISize(i) => i as $T,
                    Integer::U8(i) => i as $T,
                    Integer::U16(i) => i as $T,
                    Integer::U32(i) => i as $T,
                    Integer::U64(i) => i as $T,
                    Integer::U128(i) => i as $T,
                    Integer::USize(i) => i as $T,
                }
            }
        }
    }
}

create_into![i8];
create_into![i16];
create_into![i32];
create_into![i64];
create_into![i128];
create_into![isize];
create_into![u8];
create_into![u16];
create_into![u32];
create_into![u64];
create_into![u128];
create_into![usize];
