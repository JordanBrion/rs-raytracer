use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Material) -> Sphere<'a> {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rs = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = rs.dot(&ray.direction);
        let c = rs.dot(&rs) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return Some(HitRecord::new(temp, self, ray));
            }
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return Some(HitRecord::new(temp, self, ray));
            }
        }
        return None;
    }
}
