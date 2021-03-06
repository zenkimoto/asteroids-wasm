use quicksilver::{
    geom::Vector,
    input::Key,
    graphics::{Color, FontRenderer},
    Graphics, Input, Result,
};

use super::scene::{Scene, Transition};
use super::game_objects::player::Player;
use super::game_objects::asteroids::{Asteroid, Sizes};
use super::game_objects::hud::Hud;
use super::game_objects::game_object::GameObject;
use super::game_objects::star_field::StarField;

const NUM_ASTEROIDS: u8 = 27;
const NUM_SPAWN_ASTEROIDS: i32 = 3;

pub struct AsteroidsScene {
    window_size: Vector,
    player: Player,
    asteroids: Vec<Asteroid>,
    hud: Hud,
    score: i64,
    star_field: StarField,
    transition: Option<Transition>
}

impl AsteroidsScene {
    pub fn new(window_size: &Vector, font48: FontRenderer, font16: FontRenderer) -> Self {
        AsteroidsScene {
            window_size: window_size.clone(),
            player: Player::new(&window_size),
            asteroids: AsteroidsScene::initialize_asteroids(window_size),
            hud: Hud::new(font48, font16),
            score: 0,
            star_field: StarField::new(window_size),
            transition: None,
        }
    }

    fn initialize_asteroids(window_size: &Vector) -> Vec<Asteroid> {
        (0..NUM_ASTEROIDS).map(|i| Asteroid::new(&window_size, i < 3)).collect()
    }
}

impl Scene for AsteroidsScene {
    fn update(&mut self, _input: &mut Input) {
        let mut spawn_queue: Vec<(Sizes, Vector)> = vec![];

        // Update Player
        self.player.update();
        self.player.check_bounds();


        // If all asteroids are destroyed, re-initialize level
        if self.asteroids.iter().all(|a| a.is_dead()) {
            self.asteroids = AsteroidsScene::initialize_asteroids(&self.window_size);
        }

        // Check for Collisions
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.is_dead() {
                continue;
            }

            // Handle Collision Between Player and Asteroid
            if asteroid.check_collision(self.player.location, self.player.hit_radius) {
                self.player.handle_collsion();
            }

            // Handle Collision Between Bullet and Asteroid
            for bullet in self.player.bullets.iter_mut() {
                if bullet.is_alive() && asteroid.check_collision(bullet.location - self.player.translation, 1.0) {
                    asteroid.handle_collision();
                    bullet.handle_collision();

                    self.score += match asteroid.size {
                        Sizes::Large => 50,
                        Sizes::Medium => 100,
                        Sizes::Small => 200
                    };

                    // If an asteroid is destroyed, queue a smaller version to be spawned
                    if asteroid.size != Sizes::Small {
                        spawn_queue.push((asteroid.size, asteroid.location));
                    }
                }
            }
        }

        // Spawn Smaller Asteroids
        while let Some((size, location)) = spawn_queue.pop() {
            let mut spawn_count = 0;

            // Reuse dead asteroids for spawning
            for asteroid in self.asteroids.iter_mut() {
                if asteroid.is_alive() { continue; }

                if spawn_count < NUM_SPAWN_ASTEROIDS {
                    asteroid.spawn_asteroid(&location, &size);
                    spawn_count += 1;
                }
            }
        }

        // Update Asteroids
        for asteroid in self.asteroids.iter_mut() {
            asteroid.update();
            asteroid.check_bounds();
        }

        // Update Hud
        self.hud.set_lives(self.player.lives);
        self.hud.set_score(self.score);
        self.hud.update();
    }

    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a black
        gfx.clear(Color::BLACK);

        // Render Starfield
        self.star_field.render(gfx)?;

        // Render player and bullets
        self.player.render(gfx)?;

        // Render asteroids
        for asteroid in self.asteroids.iter_mut() {
            asteroid.render(gfx)?;
        }

        // Render hud
        self.hud.render(gfx)?;

        Ok(())
    }

    fn key_down(&mut self, key: Key) {
        match key {
            Key::Left => self.player.rotate(-4.0),
            Key::Right => self.player.rotate(4.0),
            Key::Up => self.player.apply_thrust(),
            Key::Return if !self.player.is_alive() => self.transition = Some(Transition::Reset),
            _ => { }
        }
    }

    fn key_up(&mut self, key: Key) {
        if key == Key::Space {
            self.player.shoot_bullet();
        }
    }

    fn should_transition(&self) -> bool {
        if let Some(_) = &self.transition {
            true
        } else {
            false
        }
    }

    fn get_transition(&self) -> Option<Transition> {
        self.transition.clone()
    }
}