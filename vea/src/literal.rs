#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal<'a> {
    Bool(bool),
    Integer(i64),
    String(&'a str),
}
