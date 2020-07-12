use quicksilver::{
    graphics::{Color, Graphics, FontRenderer},
    geom::Vector,
    Result
};

use crate::game_object::GameObject;
use crate::math::VectorMath;

const MARGIN: f32 = 20.0;
const CHAR_WIDTH: f32 = 8.0;

pub struct Hud {
    player_lives: i32,
    score: i64,
    object_vertices: Vec<Vector>,
    font72: FontRenderer,
    font16: FontRenderer,
}

impl Hud {
    pub fn new(font72: FontRenderer, font16: FontRenderer) -> Self {
        let object_vertices = vec![v!(0.0, 1.5), v!(-1.0, -1.0), v!(1.0, -1.0), v!(0.0, 1.5)];

        let object_vertices = object_vertices.iter()
                                             .map(|x| x.multiply(6.0))
                                             .collect();

        Self {
            player_lives: 0,
            score: 0,
            object_vertices,
            font72,
            font16,
        }
    }

    pub fn set_lives(&mut self, lives: i32) {
        self.player_lives = lives;
    }

    pub fn set_score(&mut self, score: i64) {
        self.score = score;
    }

    fn build_ship_icon(&self, location: Vector) -> Vec<Vector> {
        self.object_vertices.iter().map(|x| *x + location).collect()
    }
}

impl GameObject for Hud {
    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        // Write out Lives Label
        self.font16.draw(
            gfx,
            "LIVES",
            Color::WHITE,
            v!(14.0, 24.0)
        )?;

        for i in 0..self.player_lives {
            let top_left = Vector::new(MARGIN + (i as f32) * MARGIN, MARGIN * 2.0 - 2.0);
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

        // Write out Score Label
        self.font16.draw(
            gfx,
            "SCORE",
            Color::WHITE,
            v!(965.0, 24.0)
        )?;

        // Right Align Score
        let score_str = &format!("{}", self.score);
        let x = 1024.0 - (score_str.len() as f32) * CHAR_WIDTH - MARGIN;
        let y = MARGIN * 2.0;

        self.font16.draw(
            gfx,
            score_str,
            Color::WHITE,
            v!(x, y),
        )?;

        Ok(())
    }
}

