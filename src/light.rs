use crate::{util, vec3::Vec3};

pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }

    fn direction(&self, point: Vec3) -> Vec3 {
        (point - self.position).unit()
    }

    fn diffused_highlight(&self, direction: Vec3, normal: Vec3) -> f32 {
        let hightlight = Vec3::dot(normal, direction);
        util::max(hightlight * self.intensity, 0.0)
    }

    fn specular_highlight(
        &self,
        light_direction: Vec3,
        normal: Vec3,
        ray_direction: Vec3,
        specularity: f32,
    ) -> f32 {
        let hightlight = Vec3::dot(normal, light_direction);

        let v = -1.0 * ray_direction;
        let r = light_direction - (normal * hightlight * 2.0);

        let dot = Vec3::dot(v, r);

        util::max(dot.powf(specularity) * self.intensity, 0.0)
    }
}
