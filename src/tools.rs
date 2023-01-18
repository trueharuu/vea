use std::fmt::Debug;

pub fn do_while(body: impl Fn(), condition: &bool) {
    let mut first = true;
    while first || *condition {
        first = false;
        body();
    }
}

pub trait ResultAssumptions<T, E> {
    fn assume<F>(self) -> Result<T, F> where E: Debug;
    fn assume_err<U>(self) -> Result<U, E> where T: Debug;
}

impl<T, E> ResultAssumptions<T, E> for Result<T, E> {
    /// Assumes a `Result<T, E>` is always `Ok`, changing the type to `Result<T, F>`
    /// panics if the Result is `Err`.
    fn assume<F>(self) -> Result<T, F>
    where
        E: Debug,
    {
        Ok(self.unwrap())
    }

    /// Assumes a `Result<T, E>` is always `Err`, changing the type to `Result<U, E>`
    /// panics if the Result is `Ok`.
    fn assume_err<U>(self) -> Result<U, E>
    where
        T: Debug,
    {
        Err(self.unwrap_err())
    }
}
