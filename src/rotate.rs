use super::aabb::*;
use super::angles::*;
use super::constants::*;
use super::hittable::*;
use super::ray::*;
use super::vec3::*;
use libm::*;

use std::rc::*;
pub struct RotateY {
    ptr: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(ptr: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match ptr.bounding_box(0.0, 1.0) {
            Some(aabb) => aabb,
            _ => Default::default(),
        };
        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            let fi = i as f64;
            for j in 0..2 {
                let fj = j as f64;
                for k in 0..2 {
                    let fk = k as f64;
                    let x = fi * bbox.max.x() + (1.0 - fi) * bbox.min.x();
                    let y = fj * bbox.max.y() + (1.0 - fj) * bbox.min.y();
                    let z = fk * bbox.max.z() + (1.0 - fk) * bbox.min.z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = fmin(min[c], tester[c]);
                        max[c] = fmax(max[c], tester[c]);
                    }
                }
            }
        }

        RotateY {
            ptr: ptr,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: Some(AABB::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];
        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];
        let rotated_r = Ray::new(origin, direction, ray.time);
        if let Some(mut record) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = record.p;
            let mut normal = record.normal;
            p[0] = self.cos_theta * record.p[0] + self.sin_theta * record.p[2];
            p[2] = -self.sin_theta * record.p[0] + self.cos_theta * record.p[2];
            normal[0] = self.cos_theta * record.normal[0] + self.sin_theta * record.normal[2];
            normal[2] = -self.sin_theta * record.normal[0] + self.cos_theta * record.normal[2];
            record.p = p;
            record.set_face_normal(&rotated_r, normal);
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.bbox
    }
}
