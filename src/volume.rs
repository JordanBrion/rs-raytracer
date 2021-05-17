use std::rc::Rc;

use libm::*;

use crate::{random::random_double, vec3::Vec3};

use super::aabb::*;
use super::constants::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;

pub struct ConstantMedium<'a> {
    boundary: Rc<dyn Hittable + 'a>,
    phase_function: &'a dyn Material,
    neg_inv_density: f64,
}

impl<'a> ConstantMedium<'a> {
    pub fn new(b: Rc<dyn Hittable + 'a>, d: f64, material: &'a dyn Material) -> ConstantMedium<'a> {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: material,
        }
    }
}

impl<'a> Hittable for ConstantMedium<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut record1) = self.boundary.hit(ray, -INFINITY, INFINITY) {
            if let Some(mut record2) = self.boundary.hit(ray, record1.t + 0.0001, INFINITY) {
                record1.t = fmax(record1.t, t_min);
                record2.t = fmin(record2.t, t_max);
                if record1.t >= record2.t {
                    return None;
                } else {
                    record1.t = fmax(record1.t, 0.0);
                    let ray_length = ray.direction.length();
                    let distance_inside_boundary = (record2.t - record1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * random_double().ln();
                    if hit_distance > distance_inside_boundary {
                        return None;
                    } else {
                        let t = record1.t + hit_distance / ray_length;
                        return Some(HitRecord {
                            t: t,
                            p: ray.point_at_parameter(t),
                            normal: Vec3::new(0.0, 1.0, 0.0),
                            front_face: true,
                            u: Default::default(),
                            v: Default::default(),
                            material: self.phase_function,
                        });
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
