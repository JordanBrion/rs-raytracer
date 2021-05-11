use libm::{fmax, fmin};

use super::hittable::*;
use super::ray::*;
use super::vec3::*;

struct AABB {
    minimum: Vec3,
    maximum: Vec3,
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.minimum[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = fmax(t0, t_min);
            let t_max = fmin(t1, t_max);
            if t_max <= t_min {
                return None;
            }
        }
        Default::default()
    }
}
