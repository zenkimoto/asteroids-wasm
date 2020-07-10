use quicksilver::{
    graphics::Color,
    geom::Vector,
    Graphics, Result
};
// use quicksilver::geom::Circle;
use rand::Rng;

use crate::math::VectorMath;
use crate::game_object::GameObject;

macro_rules! rand {
    () => {
        rand::thread_rng().gen::<u32>()
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizes {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone)]
pub struct Asteroid {
    pub alive: bool,
    pub size: Sizes,
    pub hit_radius: f32,
    pub rotation: f32,
    pub location: Vector,
    pub velocity: Vector,
    pub object_vertices: Vec<Vector>,
    pub world_vertices: Vec<Vector>,
    pub translation: Vector,
}

impl Asteroid {
    pub fn new(window_size: &Vector, alive: bool) -> Self {
        let translation = window_size.divide(2.0);

        let sign_x = rand!() % 100;
        let sign_y = rand!() % 100;

        // Start Asteroid in random location
        let mut lx = rand!() as f32 % window_size.x / 2.0;
        let mut ly = rand!() as f32 % window_size.y / 2.0;

        // give asteroid random velocity
        let mut vx = (rand!() as f32 % 500.0) / 1000.0;
        let mut vy = (rand!() as f32 % 500.0) / 1000.0;

        let mut degrees = (rand!() as f32 % 100.0 + 1000.0) / 1000.0;

        // 50% chance the sign of the variable will be switched to negative
        if sign_x >= 50 {
            vx = -vx;
            lx = -lx;
            degrees = -degrees;
        }

        // 50% chance the sign of the variable will be switched to negative
        if sign_y >= 50 {
            vy = -vy;
            ly = -ly;
        }

        let mut object_vertices = Asteroid::get_object_vertices();

        let mut world_vertices: Vec<Vector> = Vec::new();

        for i in 0..object_vertices.len() {
            // converts verts from obj space to world space and translate world space to screen space
            object_vertices[i] = object_vertices[i].multiply(88.0);
            world_vertices.push(object_vertices[i] + translation);
        }

        Self {
            alive,
            size: Sizes::Large,
            hit_radius: 35.0,
            rotation: degrees,
            location: Vector::new(lx, ly),
            velocity: Vector::new(vx, vy),
            object_vertices,
            world_vertices,
            translation,
        }
    }

    fn get_object_vertices() -> Vec<Vector> {
        vec![
            Vector::new(0.0, 0.4),
            Vector::new(0.2, 0.3),
            Vector::new(0.2, 0.1),
            Vector::new(0.4, 0.0),
            Vector::new(0.3, -0.2),
            Vector::new(0.1, -0.2),
            Vector::new(0.0, -0.3),
            Vector::new(-0.2, -0.2),
            Vector::new(-0.4, 0.0),
            Vector::new(-0.3, 0.3),
            Vector::new(0.0, 0.4),
        ]
    }

    pub fn is_dead(&self) -> bool {
        !self.alive
    }

    pub fn check_bounds(&mut self) {
        let screen_width = self.translation.x;
        let screen_height = self.translation.y;

        if self.location.x < -screen_width {
            self.location.x = screen_width;
        }

        if self.location.x > screen_width {
            self.location.x = -screen_width;
        }

        if self.location.y < -screen_height {
            self.location.y = screen_height;
        }

        if self.location.y > screen_height {
            self.location.y = -screen_height;
        }
    }

    pub fn check_collision(&self, vec: Vector, radius: f32) -> bool {
        if !self.alive { return false }

        let sum = self.hit_radius + radius;
        let a = (self.location.x - vec.x).powf(2.0);
        let b = (self.location.y - vec.y).powf(2.0);

        let dist = (a + b).sqrt();

        return dist < sum;
    }

    pub fn handle_collsion(&mut self) {
        self.alive = false;
    }

    pub fn spawn_asteroid(&mut self, location: &Vector, size: &Sizes) {
        self.location = location.clone();
        self.alive = true;

        // give asteroid random velocity
        let vx = (rand!() as f32 % 500.0) / 1000.0;
        let vy = (rand!() as f32 % 500.0) / 1000.0;

        let degrees = (rand!() as f32 % 100.0 + 1000.0) / 1000.0;

        self.velocity = Vector::new(vx, vy);
        self.rotation = degrees;

        self.shrink_asteroid(size);
    }

    pub fn shrink_asteroid(&mut self, size: &Sizes) {
        let mut object_vertices = Asteroid::get_object_vertices();

        for i in 0..self.object_vertices.len() {
            // converts verts from obj space to world space and translate world space to screen space
            object_vertices[i] = object_vertices[i].multiply(88.0);
        }

        self.size = match size {
            Sizes::Large => {
                for i in 0..self.object_vertices.len() {
                    self.object_vertices[i] = object_vertices[i].divide(2.0);
                }
                self.hit_radius = 35.0 / 2.0;
                Sizes::Medium
            },
            Sizes::Medium => {
                for i in 0..self.object_vertices.len() {
                    self.object_vertices[i] = object_vertices[i].divide(4.0);
                }
                self.hit_radius = 35.0 / 4.0;
                Sizes::Small
            },
            Sizes::Small => {
                for i in 0..self.object_vertices.len() {
                    self.object_vertices[i] = object_vertices[i].divide(8.0);
                }
                self.hit_radius = 35.0 / 8.0;
                Sizes::Small
            }
        };
    }
}

impl GameObject for Asteroid {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        if self.alive {
            gfx.stroke_polygon(&self.world_vertices, Color::WHITE);

            // DEBUG: Collision Circle For Debugging
            let circle = quicksilver::geom::Circle::new(self.location + self.translation, self.hit_radius);
            gfx.stroke_circle(&circle, Color::RED);
        }

        Ok(())
    }

    fn update(&mut self) {
        self.location = self.location + self.velocity;

        for i in 0..self.object_vertices.len() {
            self.world_vertices[i] = self.object_vertices[i] + self.location + self.translation;
            self.object_vertices[i] = self.object_vertices[i].rotate(self.rotation);
        }
    }
}