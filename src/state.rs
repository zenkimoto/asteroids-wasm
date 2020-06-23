use quicksilver::{
    input::{Key},
    Graphics, Result,
};
use crate::context::Context;

pub trait State {
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context, gfx: &mut Graphics) -> Result<()>;

    fn key_down(&mut self, key: Key) {
        println!("Key Down: {:?}", key);
    }

    fn key_up(&mut self, key: Key) {
        println!("Key Up: {:?}", key);
    }
}
