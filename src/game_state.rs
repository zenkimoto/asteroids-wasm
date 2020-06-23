use quicksilver::{
    geom::{Rectangle, Vector},
    input::Key,
    graphics::Color,
    Graphics, Result,
};
use crate::context::Context;
use crate::state::State;

pub struct GameState {
    location: Vector,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            location: Vector::new(350.0, 100.0),
        }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) {

    }

    fn render(&mut self, _ctx: &mut Context, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a blank, white color
        gfx.clear(Color::WHITE);
        // Paint a blue square with a red outline in the center of our screen
        // It should have a top-left of (350, 100) and a size of (150, 100)
        let rect = Rectangle::new(self.location, Vector::new(100.0, 100.0));
        gfx.fill_rect(&rect, Color::RED);
        gfx.stroke_rect(&rect, Color::BLACK);

        Ok(())
    }

    fn key_down(&mut self, key: Key) {
        if key == Key::Right {
            self.location = Vector::new(self.location.x + 10.0, self.location.y);
        }
        if key == Key::Left {
            self.location = Vector::new(self.location.x - 10.0, self.location.y);
        }
    }
}