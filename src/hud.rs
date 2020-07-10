use quicksilver::{
    graphics::{Color, Graphics, FontRenderer},
    geom::Vector,
    Result
};

use crate::game_object::GameObject;
use crate::math::VectorMath;

pub struct Hud {
    player_lives: i32,
    object_vertices: Vec<Vector>,
}

impl Hud {
    pub fn new() -> Self {
        let object_vertices = vec![v!(0.0, 1.5), v!(-1.0, -1.0), v!(1.0, -1.0), v!(0.0, 1.5)];

        let object_vertices = object_vertices.iter()
                                             .map(|x| x.multiply(6.0))
                                             .collect();

        Self {
            player_lives: 0,
            object_vertices,
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
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        let mut offset = 0.0;
        for _ in 0..self.player_lives {
            let top_left = Vector::new(20.0 + offset, 20.0);
            let new_loc = top_left;

            let icon = self.build_ship_icon(new_loc);

            gfx.fill_polygon(&icon, Color::WHITE);

            offset += 20.0;
        }

        Ok(())
    }
}

// impl TextRenderable for Hud {
//     fn render(&self, gfx: &mut Graphics, font: &mut FontRenderer) {
//         if self.player_lives == 0 {
//             let _ = font.draw(
//                 gfx,
//                 "Game Over!",
//                 Color::WHITE,
//                 Vector::new(350.0, 400.0),
//             );
//         }
//     }
// }

