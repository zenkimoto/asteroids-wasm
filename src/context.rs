use quicksilver::{
    Input, Window,
};

pub struct Context {
    pub window: Window,
    pub input: Input,
}

impl Context {
    pub fn new(window: Window, input: Input) -> Self {
        Self {
            window,
            input,
        }
    }
}