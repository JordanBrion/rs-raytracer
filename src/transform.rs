use rand::distributions::Normal;

use super::aabb::*;
use super::hittable::*;
use super::normal::NormalOp;
use super::ray::*;
use super::vec3::*;

struct Translate<'a, T>
where
    T: Hittable + NormalOp,
{
    offset: Vec3,
    hittable: &'a T,
}

impl<'a, T> Hittable for Translate<'a, T>
where
    T: Hittable + NormalOp,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin + self.offset, ray.direction, ray.time);
        if let Some(record) = self.hittable.hit(&moved_ray, t_min, t_max) {
            let (front_facing, normal) = self.hittable.normal(&moved_ray, record.t);
            Some(HitRecord {
                front_face: front_facing,
                normal: normal,
                p: record.p + self.offset,
                ..record
            })
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(aabb) = self.hittable.bounding_box(time0, time1) {
            Some(AABB {
                minimum: aabb.minimum,
                maximum: aabb.maximum,
            })
        } else {
            None
        }
    }
}
