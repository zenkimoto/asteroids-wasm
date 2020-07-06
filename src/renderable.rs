use quicksilver::{ Graphics, Result };

pub trait Renderable {
    fn render(&self, gfx: &mut Graphics) -> Result<()>;
}
