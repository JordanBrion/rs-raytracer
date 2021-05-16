use super::aabb::*;
use super::angles::*;
use super::constants::*;
use super::hittable::*;
use super::normal::*;
use super::ray::*;
use super::vec3::*;

use libm::{fmax, fmin};

pub struct Translate<'a> {
    pub offset: Vec3,
    pub ptr: Box<dyn Hittable + 'a>,
}

pub struct RotationY<'a> {
    ptr: Box<dyn Hittable + 'a>,
    sin_theta: f64,
    cos_theta: f64,
    maybe_aabb: Option<AABB>,
}

impl<'a> RotationY<'a> {
    pub fn new(ptr: Box<dyn Hittable + 'a>, angle_deg: f64) -> RotationY<'a> {
        let radians = degrees_to_radians(angle_deg);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let maybe_aabb = match ptr.bounding_box(0.0, 1.0) {
            Some(aabb) => {
                let mut minimum = Vec3::new(INFINITY, INFINITY, INFINITY);
                let mut maximum = Vec3::new(-INFINITY, -INFINITY, -INFINITY);
                let range_max = 2;
                for i in 0..range_max {
                    let f_i = i as f64;
                    for j in 0..range_max {
                        let f_j = j as f64;
                        for k in 0..range_max {
                            let f_k = k as f64;
                            let x = f_i * aabb.maximum.x + (1.0 - f_i) * aabb.minimum.x;
                            let y = f_j * aabb.maximum.y + (1.0 - f_j) * aabb.minimum.y;
                            let z = f_k * aabb.maximum.z + (1.0 - f_k) * aabb.minimum.z;
                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(new_x, y, new_z);
                            for c in 0..3 {
                                minimum[c] = fmin(minimum[c], tester[c]);
                                maximum[c] = fmax(maximum[c], tester[c]);
                            }
                        }
                    }
                }
                Some(AABB {
                    minimum: minimum,
                    maximum: maximum,
                })
            }
            _ => None,
        };
        RotationY {
            ptr: ptr,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            maybe_aabb: maybe_aabb,
        }
    }
}

impl<'a> Hittable for Translate<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(record) = self.ptr.hit(&moved_ray, t_min, t_max) {
            let (front_facing, normal) = make_facing_normal(&moved_ray, record.normal);
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
        if let Some(aabb) = self.ptr.bounding_box(time0, time1) {
            Some(aabb)
        } else {
            None
        }
    }
}

impl<'a> Hittable for RotationY<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rotated_ray = Ray::new(
            Vec3::new(
                self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
                ray.origin.y,
                self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
            ),
            Vec3::new(
                self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
                ray.direction.y,
                self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
            ),
            ray.time,
        );
        if let Some(record) = self.ptr.hit(&rotated_ray, t_min, t_max) {
            let n = Vec3::new(
                self.cos_theta * record.normal.x + self.sin_theta * record.normal.z,
                record.normal.y,
                -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z,
            );
            let (front_facing, final_normal) = make_facing_normal(&rotated_ray, n);
            Some(HitRecord {
                p: Vec3::new(
                    self.cos_theta * record.p.x + self.sin_theta * record.p.z,
                    record.p.y,
                    -self.sin_theta * record.p.x + self.cos_theta * record.p.z,
                ),
                front_face: front_facing,
                normal: final_normal,
                ..record
            })
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.maybe_aabb.clone()
    }
}
