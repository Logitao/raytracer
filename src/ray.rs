use crate::{
    camera::Camera, color::Color, light::Light, scene::Scene, sphere::Sphere, util, vec3::Vec3,
};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn make_for_pixel(camera: &Camera, x: u32, y: u32) -> Self {
        let dy = 1.0;
        let dx = 1.0;

        let px = (-1.0 * camera.height as f32 / 2.0) + dy * (x as f32 + 0.5);
        let py = (-1.0 * camera.width as f32 / 2.0) + dx * (y as f32 + 0.5);

        let a = camera.plane_center;
        let b = camera.plane_direction_x * px;
        let c = camera.plane_direction_y * py;

        let vec = a + b + c;

        Self {
            origin: camera.position,
            direction: vec.unit(),
        }
    }

    pub fn trace(&self, scene: &Scene) -> Color {
        for sphere in scene.spheres.iter() {
            let intersection = self.intersect_sphere(&sphere);

            if let Some(distance) = intersection {
                return sphere.color;
            }
        }

        Color::new(0.0, 0.0, 0.0)
    }

    pub fn intersect_sphere(&self, sphere: &Sphere) -> Option<f32> {
        let a = Vec3::dot(self.direction, self.direction);
        let sphere_direction = self.origin - sphere.position;

        let b = 2.0 * Vec3::dot(self.direction, sphere_direction);
        let c = Vec3::dot(sphere_direction, sphere_direction) - sphere.radius.powf(2.0);

        let delta = b.powf(2.0) - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }

        let delta_sqrt = delta.sqrt();

        let q = if b < 0.0 {
            -b - delta_sqrt
        } else {
            -b + delta_sqrt
        } / 2.0;

        let max_distance = util::max(q / a, c / q);
        let min_distance = util::min(q / a, c / q);

        let distance = util::clamp(min_distance, 0.0, max_distance);

        Some(distance)
    }
}
