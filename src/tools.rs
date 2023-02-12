#[macro_export]
macro_rules! b {
    [$T:ty] => {
        Box<$T>
    };
}

