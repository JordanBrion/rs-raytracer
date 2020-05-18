use super::material::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;
use super::aabb::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new_sphere(t: f32, sphere: &'a Sphere, ray: &Ray) -> HitRecord<'a> {
        let hit_point = ray.point_at_parameter(t);
        let outward_normal = (hit_point - sphere.center) / sphere.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0f32;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            t: t,
            p: hit_point,
            normal: final_normal,
            front_face: front_face,
            material: sphere.material,
        }
    }

    pub fn new_moving_sphere(t: f32, sphere: &'a MovingSphere, ray: &Ray) -> HitRecord<'a> {
        let hit_point = ray.point_at_parameter(t);
        let outward_normal = (hit_point - sphere.center(ray.time)) / sphere.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0f32;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            t: t,
            p: hit_point,
            normal: final_normal,
            front_face: front_face,
            material: sphere.material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
