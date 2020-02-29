use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = (vfov / 180.0) * std::f64::consts::PI as f32;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let view_direction = look_at - look_from;
        let w = -view_direction.unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: view_direction - half_width * u - half_height * v,
            horizontal: half_width * 2.0 * u,
            vertical: half_height * 2.0 * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
