use super::hittable::*;
use super::ray::*;
use super::vec3::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rs = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = rs.dot(&ray.direction);
        let c = rs.dot(&rs) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius
                });
            }
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius
                });
            }
        } 
        return None;
    }
}
