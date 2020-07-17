use quicksilver::{
    graphics::{Color, Graphics, FontRenderer},
    geom::Vector,
    Result
};

use super::game_object::GameObject;
use crate::math::VectorMath;

const MARGIN: f32 = 20.0;
const CHAR_WIDTH: f32 = 8.0;

pub struct Hud {
    player_lives: i32,
    score: i64,
    object_vertices: Vec<Vector>,
    font48: FontRenderer,
    font16: FontRenderer,
    alpha: f32,
}

impl Hud {
    pub fn new(font48: FontRenderer, font16: FontRenderer) -> Self {
        let object_vertices: Vec<Vector> = vec![v!(0.0, 1.5), v!(-1.0, -1.0), v!(1.0, -1.0), v!(0.0, 1.5)];

        let object_vertices = object_vertices.iter()
                                             .map(|x| x.multiply(6.0))
                                             .collect();

        Self {
            player_lives: 0,
            score: 0,
            object_vertices,
            font48,
            font16,
            alpha: 0.0,
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

        // Draw player lives icons
        for i in 0..self.player_lives {
            let top_left = Vector::new(MARGIN + (i as f32) * MARGIN, MARGIN * 2.0 - 2.0);
            let new_loc = top_left;

            let icon = self.build_ship_icon(new_loc);

            gfx.fill_polygon(&icon, Color::WHITE);
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

        if self.player_lives == 0 {
            self.font48.draw(
                gfx,
                "Game Over!",
                Color::from_rgba(255, 255, 255, if self.alpha < 0.5 { self.alpha * 2.0 } else { 1.0 }),
                Vector::new(400.0, 397.0),
            )?;

            let x = 466.0 - ((score_str.len() as f32) * CHAR_WIDTH / 2.0);

            self.font16.draw(
                gfx,
                &format!("YOUR SCORE: {}", self.score),
                Color::from_rgba(255, 255, 255, self.alpha),
                v!(x, 435.0)
            )?;

            self.font16.draw(
                gfx,
                &format!("Press ENTER To Restart"),
                Color::from_rgba(255, 255, 255, self.alpha),
                v!(428.0, 520.0)
            )?;
        }

        Ok(())
    }

    fn update(&mut self) {
        if self.player_lives == 0 && self.alpha < 1.0 {
            self.alpha += 0.025;
        }
    }
}

