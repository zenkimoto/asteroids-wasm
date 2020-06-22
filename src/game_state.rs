use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Timer, Window,
};
use crate::context::Context;

pub struct GameState {

}

impl GameState {
    pub fn new() -> Self {
        Self { }
    }

    pub fn update(&mut self, ctx: &mut Context) {

    }

    pub fn render(&mut self, ctx: &mut Context, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a blank, white color
        gfx.clear(Color::WHITE);
        // Paint a blue square with a red outline in the center of our screen
        // It should have a top-left of (350, 100) and a size of (150, 100)
        let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
        gfx.fill_rect(&rect, Color::RED);
        gfx.stroke_rect(&rect, Color::BLACK);

        Ok(())
    }
}
