use super::hittable::*;
use super::ray::*;
use super::vec3::*;
use libm::*;

#[derive(Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
    }

   pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > t_min { t0 } else { t_min };
            let tmax = if t1 < t_max { t1 } else { t_max };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            fmin(box0.min.x(), box1.min.x()),
            fmin(box0.min.y(), box1.min.y()),
            fmin(box0.min.z(), box1.min.z()),
        );
        let big = Vec3::new(
            fmax(box0.max.x(), box1.max.x()),
            fmax(box0.max.y(), box1.max.y()),
            fmax(box0.max.z(), box1.max.z()),
        );
        AABB::new(small, big)
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB::new(Default::default(), Default::default())
    }
}
