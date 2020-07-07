use quicksilver::{
    graphics::Color,
    geom::Vector,
    Graphics, Result,
};

pub struct Player {
    pub hit_radius: f32,
    pub lives: i32,
    pub location: Vector,
    pub velocity: Vector,
    pub object_vertices: Vec<Vector>,
    pub world_vertices: Vec<Vector>,
    pub translation: Vector,
}

use crate::math::VectorMath;
use crate::game_object::GameObject;

impl Player {
    pub fn new(window_size: &Vector) -> Self {
        let translation = window_size.clone() / 2.0;

        let object_vertices = vec!(Vector::new(0.0, -18.0), Vector::new(12.0, 12.0), Vector::new(-12.0, 12.0), Vector::new(0.0, -18.0));

        let world_vertices = object_vertices.iter().map(|x| *x + translation).collect();

        Self {
            hit_radius: 15.0,
            lives: 3,
            location: Vector::ZERO,
            velocity: Vector::ZERO,
            object_vertices,
            world_vertices,
            translation,
        }
    }

    pub fn apply_force(&mut self, v: Vector) {
        self.velocity = self.velocity + v;
    }

    pub fn rotate(&mut self, degrees: f32) {
        self.object_vertices = self.object_vertices.iter().map(|x| x.rotate(degrees)).collect();
    }
}

impl GameObject for Player {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        gfx.stroke_polygon(&self.world_vertices, Color::WHITE);

        Ok(())
    }

    fn update(&mut self) {
        self.world_vertices = self.object_vertices.iter().map(|x| *x + self.location + self.translation).collect();
    }
}