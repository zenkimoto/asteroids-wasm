use quicksilver::geom::Vector;

pub struct Player {
    pub hit_radius: f32,
    pub lives: i32,
    pub location: Vector,
    pub velocity: Vector,
    pub vertices: Vec<Vector>,
}

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