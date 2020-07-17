#[macro_export]
macro_rules! v {
    ($x:expr, $y:expr) => {
        Vector::new($x, $y)
    };
}

#[macro_export]
macro_rules! rand {
    () => {
        rand::thread_rng().gen::<u16>() as i32
    };
    ($e:expr) => {
        rand::thread_rng().gen_range(0, $e) as i32
    };
    ($e:expr, $f:expr) => {
        rand::thread_rng().gen_range($e, $f) as i32
    };
}

#[macro_export]
macro_rules! randf {
    () => {
        rand::thread_rng().gen::<u16>() as f32
    };
    ($e:expr) => {
        rand::thread_rng().gen_range(0, $e) as f32
    };
    ($e:expr, $f:expr) => {
        rand::thread_rng().gen_range($e, $f) as f32
    };
}