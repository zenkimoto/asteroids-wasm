use quicksilver::{
    graphics::Color,
    geom::Vector,
    Graphics, Result
};
use rand::Rng;

use super::util::math::VectorMath;
use super::game_object::GameObject;

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
    pub explosion: Vec<(Vector, Vector, f32)>
}

impl Asteroid {
    pub fn new(window_size: &Vector, alive: bool) -> Self {
        let translation = window_size.divide(2.0);

        let object_vertices: Vec<Vector> = Asteroid::generate_vertices().iter()
                                                                          .map(|x| x.multiply(88.0))
                                                                          .collect();

        // converts verts from obj space to world space and translate world space to screen space
        let world_vertices = object_vertices.iter()
                                            .map(|x| *x + translation)
                                            .collect();

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
            explosion: vec![],
        }
    }

    fn generate_vertices() -> Vec<Vector> {
        // Randomly generate asteroid
        let mut vertices = vec![];
        let num_vertices = randf!(8, 21);
        let degree_interval = 360.0 / num_vertices;
        let is_smooth = rand!(0, 3);  // 1 in 3 chance the asteroid is smooth (more round)

        for i in 0..(num_vertices as i32) {
            let deg = degree_interval * i as f32;
            let mag = if is_smooth == 0 { randf!(0.3, 0.4) } else { randf!(0.2, 0.45) };

            let v = v!(mag, 0.0).rotate(deg);

            vertices.push(v);
        }

        vertices
    }

    fn get_random_location(window_size: &Vector) -> Vector {
        let lx = randf!() % window_size.x / 2.0;
        let ly = randf!() % window_size.y / 2.0;

        v!(lx, ly)
    }

    fn get_random_sign() -> f32 {
        2.0 * randf!(2) - 1.0
    }

    fn get_random_velocity() -> Vector {
        let vx = randf!(500) / 1000.0 * Asteroid::get_random_sign();
        let vy = randf!(500) / 1000.0 * Asteroid::get_random_sign();

        v!(vx, vy)
    }

    fn get_random_degrees() -> f32 {
        Asteroid::get_random_sign() * (randf!(100) + 1000.0) / 1000.0
    }

    pub fn is_dead(&self) -> bool {
        !self.alive
    }

    pub fn is_alive(&self) -> bool {
        self.alive
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

    pub fn handle_collision(&mut self) {
        self.alive = false;

        let size = match self.size {
            Sizes::Small => 6.0,
            Sizes::Medium => 12.0,
            Sizes::Large => 24.0,
        };

        let count = rand!(3, 9);
        self.explosion = (0..count).map(|d| (d as f32) * (360.0 / count as f32))
                                   .map(|d| d + randf!(-10, 14))
                                   .map(|d| v!(2.0, 0.0).rotate(d))
                                   .map(|v| (self.location.clone(), v, size))
                                   .collect();
    }

    pub fn spawn_asteroid(&mut self, location: &Vector, size: &Sizes) {
        self.location = location.clone();
        self.alive = true;
        self.velocity = Asteroid::get_random_velocity();
        self.rotation = Asteroid::get_random_degrees();

        self.shrink_asteroid(size);
    }

    pub fn shrink_asteroid(&mut self, size: &Sizes) {
        let object_vertices = Asteroid::generate_vertices();

        // converts verts from obj space to world space and translate world space to screen space
        let object_vertices: Vec<Vector> = object_vertices.iter().map(|x| x.multiply(88.0)).collect();

        self.size = match size {
            Sizes::Large => {
                self.object_vertices = object_vertices.iter().map(|x| x.divide(2.0)).collect();
                self.hit_radius = HIT_RADIUS / 2.0;
                Sizes::Medium
            },
            Sizes::Medium => {
                self.object_vertices = object_vertices.iter().map(|x| x.divide(4.0)).collect();
                self.hit_radius = HIT_RADIUS / 4.0;
                Sizes::Small
            },
            Sizes::Small => {
                self.object_vertices = object_vertices.iter().map(|x| x.divide(8.0)).collect();
                self.hit_radius = HIT_RADIUS / 8.0;
                Sizes::Small
            }
        };
    }
}

impl GameObject for Asteroid {
    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        if self.alive {
            gfx.stroke_polygon(&self.world_vertices, Color::from_rgba(237, 187, 153, 1.0));

            // DEBUG: Collision Circle For Debugging
            // let circle = quicksilver::geom::Circle::new(self.location + self.translation, self.hit_radius);
            // gfx.stroke_circle(&circle, Color::RED);
        }

        for (particle, _, size) in self.explosion.iter().filter(|x| x.2 > 0.05) {
            let circle = quicksilver::geom::Circle::new(*particle + self.translation, *size);

            gfx.fill_circle(&circle, Color::from_rgba(248, 196, 113, 1.0));
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

        self.explosion.iter_mut().for_each(|x| {
            x.0 += x.1;
            x.2 /= 1.6;
        });
    }
}