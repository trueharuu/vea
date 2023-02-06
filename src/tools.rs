#[macro_export]
macro_rules! b {
    [$T:ty] => {
        Box<$T>
    };

    [$T:tt] => {
      Box::new($T)
    };
}