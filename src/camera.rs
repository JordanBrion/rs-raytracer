use super::random::*;
use super::ray::*;
use super::vec3::*;
use super::angles::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        vfov_deg: f32,
        aspect: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vfov_deg);
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: look_from,
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0*half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0*half_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
