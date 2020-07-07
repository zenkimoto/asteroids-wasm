#[macro_export]
macro_rules! v {
    ($x:expr, $y:expr) => {
        Vector::new($x, $y)
    };
}
