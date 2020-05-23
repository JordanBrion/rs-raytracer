use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new_sphere(temp, self, ray));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new_sphere(temp, self, ray));
            }
        }
        return None;
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let vec_radius = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(
            self.center - vec_radius,
            self.center + vec_radius,
        ))
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(c0: Vec3, c1: Vec3, t0: f32, t1: f32, r: f32, m: Rc<dyn Material>) -> MovingSphere {
        MovingSphere {
            center0: c0,
            center1: c1,
            time0: t0,
            time1: t1,
            radius: r,
            material: m,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.squared_length();
        let half_b = oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new_moving_sphere(temp, self, ray));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new_moving_sphere(temp, self, ray));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let vec_radius = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(
            self.center(self.time0) - vec_radius,
            self.center(self.time0) + vec_radius,
        );
        let box1 = AABB::new(
            self.center(self.time1) - vec_radius,
            self.center(self.time1) + vec_radius,
        );
        Some(AABB::surrounding_box(box0, box1))
    }
}
