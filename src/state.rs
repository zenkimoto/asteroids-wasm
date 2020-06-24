use quicksilver::{
    input::{Key},
    Graphics, Input, Result,
};
// use crate::context::Context;

pub trait State {
    fn update(&mut self, input: &mut Input);
    fn render(&mut self, input: &mut Input, gfx: &mut Graphics) -> Result<()>;

    fn key_down(&mut self, key: Key) {
        println!("Key Down: {:?}", key);
    }

    fn key_up(&mut self, key: Key) {
        println!("Key Up: {:?}", key);
    }
}
