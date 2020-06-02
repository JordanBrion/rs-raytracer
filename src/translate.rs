use super::aabb::*;
use super::hittable::*;
use super::ray::*;
use super::vec3::*;

use std::rc::*;

pub struct Translate {
    pub ptr: Rc<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(mut record) = self.ptr.hit(&moved_r, t_min, t_max) {
            record.p += self.offset;
            record.set_face_normal(&moved_r, record.normal);
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if let Some(aabb) = self.ptr.bounding_box(t0, t1) {
            Some(AABB::new(aabb.min + self.offset, aabb.max + self.offset))
        } else {
            None
        }
    }
}
