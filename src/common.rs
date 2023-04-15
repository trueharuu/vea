use chumsky::span::SimpleSpan;

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

pub trait Rebox {
    fn rebox(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<T> Rebox for T {}

pub trait Tag {
    fn tag<U>(self, u: U) -> (Self, U)
    where
        Self: Sized,
    {
        (self, u)
    }
}

impl<T> Tag for T {}

#[macro_export]
macro_rules! choice {
    ($start:expr, $($rest:expr),*) => {{
      $start $(.or($rest))*
    }}
  }
