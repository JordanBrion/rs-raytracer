use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Camera {
        let theta = (vfov / 180.0) * std::f64::consts::PI as f32;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(half_width * 2.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, half_height * 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
