use std::collections::VecDeque;

use quicksilver::{
    graphics::Color,
    geom::Vector,
    Graphics, Result,
};
use rand::Rng;

use super::util::math::VectorMath;
use super::game_object::GameObject;
use super::bullet::Bullet;
use crate::randf;
use crate::rand;
use crate::v;

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
    pub exhaust: VecDeque<(Vector, f32)>,
    pub explosion: Vec<(Vector, Vector, f32)>,
}

impl Player {
    pub fn new(window_size: &Vector) -> Self {
        let translation = window_size.divide(2.0);

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
            exhaust: VecDeque::new(),
            explosion: vec![],
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

        self.generate_exhaust();
    }

    pub fn generate_exhaust(&mut self) {
        if let Some(head) = self.object_vertices.first() {
            let direction = head.normalize();

            let exhaust = *head + self.location - direction.multiply(28.0);

            self.exhaust.push_front((exhaust, 4.0));

            while self.exhaust.len() > 10 {
                self.exhaust.pop_back();
            }
        }
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
        for i in 0..self.bullets.len() {
            if self.bullets[i].location.x < 0.0 || self.bullets[i].location.x >= 1024.0 {
                self.bullets[i].alive = false;
            }
            if self.bullets[i].location.y < 0.0 || self.bullets[i].location.y >= 768.0 {
                self.bullets[i].alive = false;
            }
        }
    }

    pub fn handle_collsion(&mut self) {
        let count = 2 * (rand!(5, 10) as i32) - 1;
        self.explosion = (0..count).map(|d| (d as f32) * (360.0 / count as f32))
                                   .map(|d| d + randf!(-10, 14))
                                   .map(|d| v!(2.0, 0.0).rotate(d))
                                   .map(|v| (self.location.clone(), v, 12.0))
                                   .collect();

        self.location = Vector::ZERO;
        self.velocity = Vector::ZERO;
        self.lives = if self.lives > 0 { self.lives - 1 } else { 0 }
    }

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }

    pub fn shoot_bullet(&mut self) {
        let velocity = self.get_direction().multiply(10.1);
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
    fn render(&mut self, gfx: &mut Graphics) -> Result<()> {
        if self.is_alive() {
            gfx.stroke_polygon(&self.world_vertices, Color::from_rgba(255, 255, 255, 1.0));

            // DEBUG: Collision Circle For Debugging
            // let circle = quicksilver::geom::Circle::new(self.location + self.translation, self.hit_radius);
            // gfx.stroke_circle(&circle, Color::BLUE);

            for (exhaust, size) in self.exhaust.iter().filter(|x| x.1 > 0.1) {
                let circle = quicksilver::geom::Circle::new(*exhaust + self.translation, *size);

                gfx.fill_circle(&circle, Color::from_rgba(127, 179, 213, 1.0));
            }
        }

        for (particle, _, size) in self.explosion.iter().filter(|x| x.2 > 0.05) {
            let circle = quicksilver::geom::Circle::new(*particle + self.translation, *size);

            gfx.fill_circle(&circle, Color::from_rgba(248, 196, 113, 1.0));
        }

        for bullet in self.bullets.iter_mut().filter(|x| x.alive) {
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

        self.exhaust.iter_mut().for_each(|x| x.1 /= 1.5);

        self.explosion.iter_mut().for_each(|x| {
            x.0 += x.1;
            x.2 /= 1.4;
        });
    }
}