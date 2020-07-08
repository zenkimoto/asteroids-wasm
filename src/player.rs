use quicksilver::{
    graphics::Color,
    geom::Vector,
    Graphics, Result,
};

use crate::math::VectorMath;
use crate::game_object::GameObject;
use crate::bullet::Bullet;

const NUM_BULLETS: usize = 20;

pub struct Player {
    pub hit_radius: f32,
    pub lives: i32,
    pub location: Vector,
    pub velocity: Vector,
    pub object_vertices: Vec<Vector>,
    pub world_vertices: Vec<Vector>,
    pub translation: Vector,
    pub bullets: Vec<Bullet>,
}

impl Player {
    pub fn new(window_size: &Vector) -> Self {
        let translation = window_size.clone() / 2.0;

        let object_vertices = vec!(v!(0.0, -18.0), v!(12.0, 12.0), v!(-12.0, 12.0), v!(0.0, -18.0));

        let world_vertices = object_vertices.iter()
                                            .map(|x| *x + translation)
                                            .collect();

        Self {
            hit_radius: 15.0,
            lives: 3,
            location: Vector::ZERO,
            velocity: Vector::ZERO,
            object_vertices,
            world_vertices,
            translation,
            bullets: vec![Bullet::new(); NUM_BULLETS],
        }
    }

    pub fn apply_force(&mut self, v: Vector) {
        self.velocity = self.velocity + v;
    }

    pub fn rotate(&mut self, degrees: f32) {
        self.object_vertices = self.object_vertices.iter()
                                                   .map(|x| x.rotate(degrees))
                                                   .collect();
    }

    pub fn get_direction(&self) -> Vector {
        if let Some(direction) = self.object_vertices.first() {
            direction.normalize()
        } else {
            Vector::ZERO
        }
    }

    pub fn apply_thrust(&mut self) {
        let direction = self.get_direction();
        // println!("Ship Direction: {:?}", direction);
        let thrust = direction.multiply(0.6);
        // println!("Applying Thrust: {:?}", thrust);
        self.apply_force(thrust);
    }

    pub fn check_bounds(&mut self) {
        let screen_width = self.translation.x;
        let screen_height = self.translation.y;

        // println!("screen_width: {:?}", screen_width);
        // println!("screen_height: {:?}", screen_height);

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

        // bullet is out of bounds, reset bullet to be shot again
        // bullets are in world space
        for bullet in self.bullets.iter_mut() {
            if bullet.location.x < 0.0 || self.location.x >= 1024.0 {
                bullet.alive = false;
            } else if bullet.location.y < 0.0 || self.location.y >= 768.0 {
                bullet.alive = false;
            }
        }
    }

    pub fn handle_collsion(&mut self) {
        self.location = Vector::ZERO;
        self.velocity = Vector::ZERO;
        self.lives = if self.lives > 0 { self.lives - 1 } else { 0 }
    }

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }

    pub fn shoot_bullet(&mut self) {
        let velocity = self.get_direction().multiply(8.1);
        let location = self.world_vertices.first().unwrap().clone();

        for bullet in self.bullets.iter_mut().filter(|x| !x.alive) {
            bullet.alive = true;
            bullet.location = location;
            bullet.velocity = velocity;
            break;
        }
    }

}

impl GameObject for Player {
    fn render(&self, gfx: &mut Graphics) -> Result<()> {
        if self.is_alive() {
            gfx.stroke_polygon(&self.world_vertices, Color::WHITE);

            // DEBUG: Collision Circle For Debugging
            let circle = quicksilver::geom::Circle::new(self.location + self.translation, self.hit_radius);
            gfx.stroke_circle(&circle, Color::BLUE);
        }

        for bullet in self.bullets.iter().filter(|x| x.alive) {
            bullet.render(gfx)?;
        }

        Ok(())
    }

    fn update(&mut self) {
        self.velocity = self.velocity.limit(4.0);
        self.location = self.location + self.velocity;

        // println!("Ship Location: {:?}", self.location);
        // println!("Ship Velocity: {:?}", self.velocity);

        self.world_vertices = self.object_vertices.iter()
                                                  .map(|x| *x + self.location + self.translation)
                                                  .collect();

        self.bullets.iter_mut().filter(|x| x.alive).for_each(|x| x.location = x.location + x.velocity);
    }
}