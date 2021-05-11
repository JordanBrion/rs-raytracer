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
            let t0 = fmin(
                self.minimum[i] - ray.origin[i] / ray.direction[i],
                self.maximum[i] - ray.origin[i] / ray.direction[i],
            );
            let t1 = fmax(
                self.minimum[i] - ray.origin[i] / ray.direction[i],
                self.maximum[i] - ray.origin[i] / ray.direction[i],
            );
            let t_min = fmax(t0, t_min);
            let t_max = fmin(t1, t_max);
            if t_max <= t_min {
                return None;
            }
        }
        Default::default()
    }
}
