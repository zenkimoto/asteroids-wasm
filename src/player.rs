use quicksilver::{
    input::Event,
    geom::Vector,
    graphics::Color,
    run, Graphics, Input, Result, Settings, Timer, Window,
};

pub struct Player {
    pub hit_radius: f32,
    pub lives: i32,
    pub location: Vector,
    pub velocity: Vector,
    pub vertices: Vec<Vector>,
}

use crate::renderable::Renderable;

impl Player {
    pub fn new(location: Vector) -> Self {
        Self {
            hit_radius: 15.0,
            lives: 3,
            location,
            velocity: Vector::new(0.0, 0.0),
            vertices: vec!(Vector::new(0.0, 18.0), Vector::new(-12.0, -12.0), Vector::new(12.0, -12.0), Vector::new(0.0, 18.0)),
        }
    }
}

impl Renderable for Player {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        gfx.stroke_polygon(&self.vertices, Color::WHITE);

        Ok(())
    }
}