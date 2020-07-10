use quicksilver::{
    geom::Vector,
    input::Key,
    graphics::Color,
    Graphics, Input, Result,
};

use crate::state::State;
use crate::player::Player;
use crate::asteroids::Asteroid;
use crate::game_object::GameObject;

const NUM_ASTEROIDS: u8 = 27;

pub struct GameState {
    window_size: Vector,
    player: Player,
    asteroids: Vec<Asteroid>
}

impl GameState {
    pub fn new(window_size: &Vector) -> Self {
        Self {
            window_size: window_size.clone(),
            player: Player::new(&window_size),
            asteroids: GameState::initialize_asteroids(window_size)
        }
    }

    fn initialize_asteroids(window_size: &Vector) -> Vec<Asteroid> {
        (0..NUM_ASTEROIDS).map(|i| Asteroid::new(&window_size, i < 3)).collect()
    }
}

impl State for GameState {
    fn update(&mut self, _input: &mut Input) {
        self.player.update();
        self.player.check_bounds();

        for asteroid in self.asteroids.iter_mut() {
            asteroid.update();
            asteroid.check_bounds();
        }
    }

    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a black
        gfx.clear(Color::BLACK);

        self.player.render(gfx)?;

        for asteroid in self.asteroids.iter_mut() {
            asteroid.render(gfx)?;
        }

        Ok(())
    }

    fn key_down(&mut self, key: Key) {
        match key {
            Key::Left => self.player.rotate(-4.0),
            Key::Right => self.player.rotate(4.0),
            Key::Up => self.player.apply_thrust(),
            _ => { }
        }
    }

    fn key_up(&mut self, key: Key) {
        if key == Key::Space {
            self.player.shoot_bullet();
        }
    }
}