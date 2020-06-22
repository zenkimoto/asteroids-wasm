use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Timer, Window,
};

pub struct Context {
    pub window: Window,
    pub input: Input,
}

impl Context {
    pub fn new(window: Window, input: Input) -> Self {
        Self {
            window,
            input
        }
    }
}