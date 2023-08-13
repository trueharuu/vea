use chumsky::span::SimpleSpan;
use std::fmt::{Debug, Display};

#[allow(clippy::module_name_repetitions)]
pub type RawSpan = SimpleSpan<usize>;

pub type RawSpanned<T> = (T, RawSpan);

pub struct Span<T>(pub T, pub RawSpan);

impl<T> Display for Span<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<RawSpanned<T>> for Span<T> {
    fn from(value: RawSpanned<T>) -> Self {
        Self(value.0, value.1)
    }
}

impl<T> Copy for Span<T> where T: Copy {}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T> Clone for Span<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T> Default for Span<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default(), (0..0).into())
    }
}

impl<T> PartialEq for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T> Debug for Span<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?} @ {}", self.0, self.1)
        } else {
            write!(f, "{:?} @ {}", self.0, self.1)
        }
    }
}

impl<T> chumsky::span::Span for Span<T>
where
    T: Clone,
{
    type Context = T;
    type Offset = usize;
    fn new(context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self(context, range.into())
    }

    fn start(&self) -> Self::Offset {
        self.1.start
    }

    fn context(&self) -> Self::Context {
        self.0.clone()
    }

    fn end(&self) -> Self::Offset {
        self.1.end
    }
}
