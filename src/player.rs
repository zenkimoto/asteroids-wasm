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
    pub object_verticies: Vec<Vector>,
    pub world_vertices: Vec<Vector>,
    pub translation: Vector,
}

use crate::renderable::Renderable;

impl Player {
    pub fn new(window_size: &Vector) -> Self {
        let translation = window_size.clone() / 2.0;

        let object_verticies = vec!(Vector::new(0.0, -18.0), Vector::new(12.0, 12.0), Vector::new(-12.0, 12.0), Vector::new(0.0, -18.0));

        let world_vertices = object_verticies.iter().map(|x| *x + translation).collect();

        Self {
            hit_radius: 15.0,
            lives: 3,
            location: Vector::ZERO,
            velocity: Vector::ZERO,
            object_verticies,
            world_vertices,
            translation,
        }
    }
}

impl Renderable for Player {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        gfx.stroke_polygon(&self.world_vertices, Color::WHITE);

        Ok(())
    }
}