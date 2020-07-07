use core::f32::consts::PI;
use quicksilver::geom::Vector;

pub trait VectorMath {
    fn multiply(&self, rhs: f32) -> Vector;
    fn divide(&self, rhs: f32) -> Vector;
    fn rotate(&self, degrees: f32) -> Vector;
    fn magnitude(&self) -> f32;
    fn normalize(&self) -> Vector;
    fn limit(&self, limit: f32) -> Vector;
}

impl VectorMath for Vector {
    fn multiply(&self, rhs: f32) -> Vector {
        v!(&self.x * rhs, &self.y * rhs)
    }

    fn divide(&self, rhs: f32) -> Vector {
        v!(&self.x / rhs, &self.y / rhs)
    }

    fn rotate(&self, degrees: f32) -> Vector {
        let angle = degrees * PI / 180.0;
        let sin = angle.sin();
        let cos = angle.cos();

        Vector::new(
            cos * self.x + -sin * self.y,
            sin * self.x + cos * self.y
        )
    }

    fn magnitude(&self) -> f32 {
        let c2 = &self.x.powf(2.0) + &self.y.powf(2.0);
        c2.sqrt()
    }

    fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        self.divide(mag)
    }

    fn limit(&self, limit: f32) -> Vector {
        let mag = self.magnitude();

        if mag > limit {
            let ratio = limit / mag;
            Self {
                x: self.x * ratio,
                y: self.y * ratio,
            }
        } else {
            self.clone()
        }
    }
}