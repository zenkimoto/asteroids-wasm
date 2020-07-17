use quicksilver::{ Graphics, Result };

pub trait GameObject {
    fn render(&mut self, gfx: &mut Graphics) -> Result<()>;
    fn update(&mut self) { }
}
