use super::angles::*;
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
    pub lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov_deg: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov_deg);
        let half_height = (theta * 0.5).tan() as f64;
        let half_width = aspect * half_height;
        let ww = (look_from - look_at).unit_vector();
        let uu = vup.cross(&ww).unit_vector();
        let vv = ww.cross(&uu);
        Camera {
            origin: look_from,
            lower_left_corner: look_from
                - half_width * focus_dist * uu
                - half_height * focus_dist * vv
                - focus_dist * ww,
            horizontal: 2.0 * half_width * focus_dist * uu,
            vertical: 2.0 * half_height * focus_dist * vv,
            u: uu,
            v: vv,
            w: ww,
            lens_radius: aperture * 0.5,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_double_in_limit(self.time0, self.time1),
        )
    }
}
