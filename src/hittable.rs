use std::rc::Rc;

use super::aabb::*;
use super::material::*;
use super::ray::*;
use super::sphere::*;
use super::texture::*;
use super::vec3::*;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new_sphere(t: f64, sphere: &Sphere, ray: &Ray) -> HitRecord {
        let hit_point = ray.point_at_parameter(t);
        let outward_normal = (hit_point - sphere.center) / sphere.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0f64;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        let (u, v) = get_sphere_uv(hit_point);
        HitRecord {
            t: t,
            p: hit_point,
            normal: final_normal,
            u: u,
            v: v,
            front_face: front_face,
            material: sphere.material.clone(),
        }
    }

    pub fn new_moving_sphere(t: f64, sphere: &MovingSphere, ray: &Ray) -> HitRecord {
        let hit_point = ray.point_at_parameter(t);
        let outward_normal = (hit_point - sphere.center(ray.time)) / sphere.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0f64;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        let (u, v) = get_sphere_uv(hit_point);
        HitRecord {
            t: t,
            p: hit_point,
            normal: final_normal,
            u: u,
            v: v,
            front_face: front_face,
            material: sphere.material.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
