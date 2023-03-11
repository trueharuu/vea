use std::ops::{AddAssign, SubAssign};

pub trait Named {
    fn name(&self) -> String;
}

pub trait Increment
where
    Self: Sized + AddAssign,
{
    const UNIT: Self;

    /// ++x
    fn incr_pre(&mut self) -> &mut Self {
        *self += Self::UNIT;
        self
    }

    /// x++
    fn incr_post(&mut self) -> Self
    where
        Self: Clone,
    {
        let temp = self.clone();
        *self += Self::UNIT;
        temp
    }
}

pub trait Decrement
where
    Self: Sized + SubAssign,
{
    const UNIT: Self;

    /// --x
    fn decr_pre(&mut self) -> &mut Self {
        *self -= Self::UNIT;
        self
    }

    /// x--
    fn decr_post(&mut self) -> Self
    where
        Self: Clone,
    {
        let temp = self.clone();
        *self -= Self::UNIT;
        temp
    }
}
