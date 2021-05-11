use libm::{fmax, fmin};

use super::hittable::*;
use super::ray::*;
use super::vec3::*;

pub struct AABB {
    minimum: Vec3,
    maximum: Vec3,
}

impl AABB {
    pub fn surrounding_box(aabb0: AABB, aabb1: AABB) -> AABB {
        AABB {
            minimum: Vec3::new(
                fmin(aabb0.minimum.x, aabb1.minimum.x),
                fmin(aabb0.minimum.y, aabb1.minimum.y),
                fmin(aabb0.minimum.z, aabb1.minimum.z),
            ),
            maximum: Vec3::new(
                fmax(aabb0.maximum.x, aabb1.maximum.x),
                fmax(aabb0.maximum.y, aabb1.maximum.y),
                fmax(aabb0.maximum.z, aabb1.maximum.z),
            ),
        }
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB {
            minimum: Default::default(),
            maximum: Default::default(),
        }
    }
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.minimum,
            maximum: self.maximum
        })
    }
}
