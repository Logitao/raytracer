use crate::{color::Color, vec3::Vec3};

pub struct Sphere {
    pub radius: f32,
    pub position: Vec3,
    pub color: Color,
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3, color: Color) -> Self {
        Self {
            radius,
            position,
            color,
        }
    }
}
