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
        let mut object_vertices = vec!(Vector::new(0.0, 1.5), Vector::new(-1.0, -1.0), Vector::new(1.0, -1.0), Vector::new(0.0, 1.5));

        for i in 0..object_vertices.len() {
            object_vertices[i] = object_vertices[i].multiply(6.0);
        }

        Self {
            player_lives: 3,
            object_vertices,
        }
    }

    pub fn set_lives(&mut self, lives: i32) {
        self.player_lives = lives;
    }

    fn build_ship_icon(&self, location: Vector) -> Vec<Vector> {
        let mut icon = self.object_vertices.clone();

        for i in 0..icon.len() {
            icon[i] += location;
        }

        icon
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

