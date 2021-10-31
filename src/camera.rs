use crate::vec3::Vec3;
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub plane_center: Vec3,
    pub plane_direction_x: Vec3,
    pub plane_direction_y: Vec3,
    pub focal_length: f32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn setup(
        position: Vec3,
        direction: Vec3,
        focal_length: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let up = Vec3::new(0.0, -1.0, 0.0);
        let plane_center = position + (focal_length * direction);
        let plane_direction_x = Vec3::normalize(Vec3::cross(direction * -1.0, up));
        let plane_direction_y = Vec3::cross(direction * -1.0, plane_direction_x);

        Self {
            position,
            direction,
            plane_center,
            plane_direction_x,
            plane_direction_y,
            focal_length,
            width,
            height,
        }
    }
}
