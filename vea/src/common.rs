use std::fmt::Display;

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

    // fn j(self, u: ) -> Span<Self> {}

    // fn tag(self, u: Span) -> Spanned<Self> where Self: Sized + std::fmt::Debug { Spanned(self, u) }
}

impl<T> Tag for T {}

#[macro_export]
macro_rules! choice {
    ($start:expr, $($rest:expr),*$(,)?) => {{
      $start $(.or($rest))*
    }}
  }

#[macro_export]
macro_rules! choice_just {
    ($start:expr, $($rest:expr),*) => {
        just($start)$(.or(just($rest)))*
    };
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum VeaErr {
    IntegerOverflow,
    InvalidStringEscape,
    InvalidQuotationMark(char),
}

impl Display for VeaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IntegerOverflow => "integer overflow".to_string(),
                Self::InvalidQuotationMark(c) => format!("invalid quote mark `{c}`"),
                Self::InvalidStringEscape => "invalid string escape".to_string()
            }
        )
    }
}

#[macro_export]
macro_rules! void {
    ($T:expr) => {{
        $T;
    }};
}

pub trait Unbox {
    type T;
    fn via_copy(&self) -> Self::T
    where
        Self::T: Copy;
    fn via_clone(&self) -> Self::T
    where
        Self::T: Clone;
    fn via_fn<F>(&self, f: F) -> Self::T
    where
        F: FnOnce(&Self) -> Self::T,
        Self: Sized,
    {
        f(self)
    }
}

impl<T> Unbox for Box<T> {
    type T = T;
    fn via_copy(&self) -> Self::T
    where
        Self::T: Copy,
    {
        **self
    }

    fn via_clone(&self) -> Self::T
    where
        Self::T: Clone,
    {
        *self.clone()
    }
}
