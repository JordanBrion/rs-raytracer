use super::hittable::*;
use super::ray::*;
use super::vec3::*;
use libm::*;

struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0 = fmin(
                ((self.min[a] - ray.origin[a]) / ray.direction[a]) as f64,
                ((self.max[a] - ray.origin[a]) / ray.direction[a]) as f64,
            );
            let t1 = fmax(
                ((self.min[a] - ray.origin[a]) / ray.direction[a]) as f64,
                ((self.max[a] - ray.origin[a]) / ray.direction[a]) as f64,
            );
            let t_min = fmax(t0 as f64, t_min as f64);
            let t_max = fmin(t1 as f64, t_max as f64);
            if t_max <= t_min {
                return false;
            } 
        }
        true
    }
}
