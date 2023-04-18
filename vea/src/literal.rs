#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Bool(bool),
    Integer(i64),
}
