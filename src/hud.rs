use quicksilver::{
    graphics::{Color, Graphics, FontRenderer},
    geom::Vector,
    Result
};

use crate::game_object::GameObject;
use crate::math::VectorMath;

const MARGIN: f32 = 20.0;

pub struct Hud {
    player_lives: i32,
    object_vertices: Vec<Vector>,
    font72: FontRenderer,
}

impl Hud {
    pub fn new(font72: FontRenderer) -> Self {
        let object_vertices = vec![v!(0.0, 1.5), v!(-1.0, -1.0), v!(1.0, -1.0), v!(0.0, 1.5)];

        let object_vertices = object_vertices.iter()
                                             .map(|x| x.multiply(6.0))
                                             .collect();

        Self {
            player_lives: 0,
            object_vertices,
            font72,
        }
    }

    pub fn set_lives(&mut self, lives: i32) {
        self.player_lives = lives;
    }

    fn build_ship_icon(&self, location: Vector) -> Vec<Vector> {
        self.object_vertices.iter().map(|x| *x + location).collect()
    }
}

impl GameObject for Hud {
    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        for i in 0..self.player_lives {
            let top_left = Vector::new(MARGIN + (i as f32) * MARGIN, MARGIN);
            let new_loc = top_left;

            let icon = self.build_ship_icon(new_loc);

            gfx.fill_polygon(&icon, Color::WHITE);
        }

        if self.player_lives == 0 {
            self.font72.draw(
                gfx,
                "Game Over!",
                Color::WHITE,
                Vector::new(350.0, 400.0),
            )?;
        }

        Ok(())
    }
}

