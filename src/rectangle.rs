use rand::distributions::Normal;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::normal::*;
use super::ray::*;
use super::uv::*;
use super::vec3::*;

struct XyRect<'a> {
    mp: &'a dyn Material,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl<'a> Hittable for XyRect<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x + t * ray.direction.x;
            let y = ray.origin.y + t * ray.direction.y;
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                Some(HitRecord::new(t, self, ray))
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let delta = 0.0001;
        Some(AABB {
            minimum: Vec3::new(self.x0, self.y0, self.k - delta),
            maximum: Vec3::new(self.x1, self.y1, self.k + delta),
        })
    }
}

impl<'a> NormalOp for XyRect<'a> {
    fn outward_normal(&self, _ray: &Ray, _t: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    fn normal(&self, ray: &Ray, t: f64) -> (bool, Vec3) {
        let outward_normal = self.outward_normal(ray, t);
        let front_face = ray.direction.dot(&outward_normal) < 0.0f64;
        (
            front_face,
            match front_face {
                true => outward_normal,
                false => -outward_normal,
            },
        )
    }
}

impl<'a> UvOp for XyRect<'a> {
    fn get_u(&self, p: Vec3) -> f64 {
        (p.x - self.x0) / (self.x1 - self.x0)
    }
    fn get_v(&self, p: Vec3) -> f64 {
        (p.y - self.y0) / (self.y1 - self.y0)
    }
}

impl<'a> MaterialOp for XyRect<'a> {
    fn material(&self) -> &dyn Material {
        self.mp
    }
}
