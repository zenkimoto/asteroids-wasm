use quicksilver::{
    geom::Vector,
    input::Key,
    graphics::Color,
    Graphics, Input, Result,
};

use crate::state::State;
use crate::player::Player;
use crate::game_object::GameObject;

pub struct GameState {
    window_size: Vector,
    player: Player,
}

impl GameState {
    pub fn new(window_size: &Vector) -> Self {
        Self {
            window_size: window_size.clone(),
            player: Player::new(&window_size),
        }
    }
}

impl State for GameState {
    fn update(&mut self, _input: &mut Input) {
        self.player.update();
        self.player.check_bounds();
    }

    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a black
        gfx.clear(Color::BLACK);

        self.player.render(gfx)?;

        Ok(())
    }

    fn key_down(&mut self, key: Key) {
        if key == Key::Left {
            self.player.rotate(-4.0);
        }
        if key == Key::Right {
            self.player.rotate(4.0);
        }
        if key == Key::Up {
            self.player.apply_thrust();
        }
    }
}