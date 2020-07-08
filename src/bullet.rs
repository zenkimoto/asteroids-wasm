use quicksilver::{
    graphics::{Color, Graphics},
    geom::{Circle, Vector},
    Result,
};

use crate::game_object::GameObject;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub location: Vector,
    pub velocity: Vector,
    pub alive: bool,
}

impl Bullet {
    pub fn new() -> Self {
        Self {
            location: Vector::ZERO,
            velocity: Vector::ZERO,
            alive: false,
        }
    }

    pub fn handle_collision(&mut self) {
        self.alive = false;
    }
}

impl GameObject for Bullet {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        let circle = Circle::new(self.location, 1.5);
        gfx.fill_circle(&circle, Color::WHITE);

        Ok(())
    }
}