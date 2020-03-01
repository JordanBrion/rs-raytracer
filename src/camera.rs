use super::random::*;
use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = (vfov / 180.0) * std::f64::consts::PI as f32;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;
        let view_direction = look_at - look_from;
        let ww = -view_direction.unit_vector();
        let uu = v_up.cross(&ww).unit_vector();
        let vv = ww.cross(&uu);
        Camera {
            lower_left_corner: look_from - half_width * focus_dist * uu - half_height * focus_dist * vv - focus_dist * ww,
            horizontal: 2.0 * half_width * focus_dist * uu,
            vertical: 2.0 * half_height * focus_dist * vv,
            origin: look_from,
            u: uu,
            v: vv,
            w: ww,
            lens_radius: aperture * 0.5,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
