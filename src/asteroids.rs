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
        rand::thread_rng().gen::<u16>() as f32
    };
    ($e:expr) => {
        rand::thread_rng().gen_range(0, $e) as f32
    };
    ($e:expr, $f:expr) => {
        rand::thread_rng().gen_range($e, $f) as f32
    };
}

const HIT_RADIUS: f32 = 35.0;

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
            hit_radius: HIT_RADIUS,
            rotation: Asteroid::get_random_degrees(),
            location: Asteroid::get_random_location(&window_size),
            velocity: Asteroid::get_random_velocity(),
            object_vertices,
            world_vertices,
            translation,
        }
    }

    fn get_object_vertices() -> Vec<Vector> {
        vec![
            v!(0.0, 0.4),
            v!(0.2, 0.3),
            v!(0.2, 0.1),
            v!(0.4, 0.0),
            v!(0.3, -0.2),
            v!(0.1, -0.2),
            v!(0.0, -0.3),
            v!(-0.2, -0.2),
            v!(-0.4, 0.0),
            v!(-0.3, 0.3),
            v!(0.0, 0.4),
        ]
    }

    fn get_random_location(window_size: &Vector) -> Vector {
        let lx = rand!() % window_size.x / 2.0;
        let ly = rand!() % window_size.y / 2.0;

        v!(lx, ly)
    }

    fn get_random_sign() -> f32 {
        2.0 * rand!(2) - 1.0
    }

    fn get_random_velocity() -> Vector {
        let vx = rand!(500) / 1000.0 * Asteroid::get_random_sign();
        let vy = rand!(500) / 1000.0 * Asteroid::get_random_sign();

        v!(vx, vy)
    }

    fn get_random_degrees() -> f32 {
        Asteroid::get_random_sign() * (rand!(100) + 1000.0) / 1000.0
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
        self.velocity = Asteroid::get_random_velocity();
        self.rotation = Asteroid::get_random_degrees();

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
                self.hit_radius = HIT_RADIUS / 2.0;
                Sizes::Medium
            },
            Sizes::Medium => {
                for i in 0..self.object_vertices.len() {
                    self.object_vertices[i] = object_vertices[i].divide(4.0);
                }
                self.hit_radius = HIT_RADIUS / 4.0;
                Sizes::Small
            },
            Sizes::Small => {
                for i in 0..self.object_vertices.len() {
                    self.object_vertices[i] = object_vertices[i].divide(8.0);
                }
                self.hit_radius = HIT_RADIUS / 8.0;
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
        // Move asteroid's location based on current velocity vector
        self.location = self.location + self.velocity;

        // Translate object vertices to world vertices for rendering
        self.world_vertices = self.object_vertices.iter()
                                                  .map(|x| *x + self.location + self.translation)
                                                  .collect();

        // Rotate asteroid for next render/update cycle
        self.object_vertices = self.object_vertices.iter()
                                                   .map(|x| x.rotate(self.rotation))
                                                   .collect();
    }
}