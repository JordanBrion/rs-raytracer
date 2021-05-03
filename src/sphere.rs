use num::traits::Pow;

use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f64, material: &'a dyn Material) -> Sphere<'a> {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new(temp, self, ray));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new(temp, self, ray));
            }
        }
        return None;
    }
}