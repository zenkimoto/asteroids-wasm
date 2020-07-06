use quicksilver::{
    geom::{Rectangle, Vector},
    input::Key,
    graphics::Color,
    Graphics, Input, Result,
};
use crate::state::State;
use crate::player::Player;

pub struct GameState {
    location: Vector,
    window_size: Vector,
    player: Player,
}

impl GameState {
    pub fn new(window_size: &Vector) -> Self {
        Self {
            location: Vector::new(350.0, 100.0),
            window_size: window_size.clone(),
            player: Player::new(Vector::new(window_size.x / 2.0, window_size.y / 2.0)),
        }
    }
}

impl State for GameState {
    fn update(&mut self, _input: &mut Input) {

    }

    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a black
        gfx.clear(Color::BLACK);
        // Paint a blue square with a red outline in the center of our screen
        // It should have a top-left of (350, 100) and a size of (150, 100)
        let rect = Rectangle::new(self.location, Vector::new(100.0, 100.0));
        gfx.fill_rect(&rect, Color::RED);
        gfx.stroke_rect(&rect, Color::BLACK);

        gfx.stroke_polygon(&self.player.vertices, Color::WHITE);

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