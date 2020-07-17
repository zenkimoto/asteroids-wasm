use quicksilver::{
    graphics::{Color, Graphics},
    geom::{Circle, Vector},
    Result,
};
use rand::Rng;

use crate::game_object::GameObject;

pub struct StarField {
    stars: Vec<Vector>,
}

impl StarField {
    pub fn new(window_size: &Vector) -> Self {
        Self {
            stars: StarField::generate_stars(window_size)
        }
    }

    fn generate_stars(window_size: &Vector) -> Vec<Vector> {
        let num_stars = rand!(45, 90);

        (0..num_stars).map(|_| v!(randf!(0.0, window_size.x), randf!(0.0, window_size.y))).collect()
    }
}

impl GameObject for StarField {
    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        let mut is_distant = false;
        for s in &self.stars {
            if is_distant {
                let circle = Circle::new(s.clone(), 0.5);
                gfx.fill_circle(&circle, Color::from_rgba(200, 200, 200, 0.8));
            } else {
                let circle = Circle::new(s.clone(), 0.8);
                gfx.fill_circle(&circle, Color::from_rgba(255, 255, 255, 1.0));
            }

            is_distant = !is_distant;
        }

        Ok(())
    }
}