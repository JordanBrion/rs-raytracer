use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

use std::rc::Rc;
pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: Rc<dyn Material>,
}

pub struct XzRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: Rc<dyn Material>,
}

pub struct YzRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: Rc<dyn Material>,
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

impl Hittable for XzRect {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x() + t * ray.direction.x();
            let z = ray.origin.z() + t * ray.direction.z();
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord::new_rect_xz(t, self, ray, x, z))
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001 , self.z1)
        ))
    }
}

impl Hittable for YzRect {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            None
        } else {
            let y = ray.origin.y() + t * ray.direction.y();
            let z = ray.origin.z() + t * ray.direction.z();
            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord::new_rect_yz(t, self, ray, y, z))
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}