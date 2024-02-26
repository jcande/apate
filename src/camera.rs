use crate::point::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3) -> Self {
        Self {
            origin: origin,
            target: Vec3::zeroes(),
            up: Vec3::new([0.0, 1.0, 0.0]),
        }
    }
}
