use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

use std::rc::Rc;
pub struct XyRect {
    pub mp: Rc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x() + t * ray.direction.x();
            let y = ray.origin.y() + t * ray.direction.y();
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                Some(HitRecord::new_rect_xy(t, self, ray, x, y))
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
