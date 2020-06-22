use quicksilver::{
    Graphics, Result,
};
use crate::context::Context;

pub trait State {
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context, gfx: &mut Graphics) -> Result<()>;
}
