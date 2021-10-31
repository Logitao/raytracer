use crate::{camera::Camera, light::Light, sphere::Sphere, vec3::Vec3};

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub surfaces: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl Scene {
    pub fn init(width: u32, height: u32, spheres: Vec<Sphere>, lights: Vec<Light>) -> Self {
        Self {
            spheres,
            lights,
            surfaces: vec![],
            camera: Camera::setup(
                Vec3::zero(),
                Vec3::new(0.0, 0.0, 1.0),
                1000.0,
                width,
                height,
            ),
        }
    }
}
