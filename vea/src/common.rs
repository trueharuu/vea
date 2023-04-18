use crate::span::RawSpan;
use crate::span::Span;

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

    fn t(self, u: RawSpan) -> Span<Self>
    where
        Self: Sized,
    {
        Span(self, u)
    }

    // fn tag(self, u: Span) -> Spanned<Self> where Self: Sized + std::fmt::Debug { Spanned(self, u) }
}

impl<T> Tag for T {}

#[macro_export]
macro_rules! choice {
    ($start:expr, $($rest:expr),*) => {{
      $start $(.or($rest))*
    }}
  }
