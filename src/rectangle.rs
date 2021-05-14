use rand::distributions::Normal;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::normal::*;
use super::ray::*;
use super::uv::*;
use super::vec3::*;

pub struct XyRect<'a> {
    pub mp: &'a dyn Material,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

pub struct XzRect<'a> {
    pub mp: &'a dyn Material,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

pub struct YzRect<'a> {
    pub mp: &'a dyn Material,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
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

impl<'a> Hittable for XzRect<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord::new(t, self, ray))
            }
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let delta = 0.0001;
        Some(AABB {
            minimum: Vec3::new(self.x0, self.k - delta, self.z0),
            maximum: Vec3::new(self.x1, self.k + delta, self.z1),
        })
    }
}

impl<'a> Hittable for YzRect<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            None
        } else {
            let y = ray.origin.y + t * ray.direction.y;
            let z = ray.origin.z + t * ray.direction.z;
            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord::new(t, self, ray))
            }
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let delta = 0.0001;
        Some(AABB {
            minimum: Vec3::new(self.k - delta, self.y0, self.z0),
            maximum: Vec3::new(self.k + delta, self.y1, self.z1),
        })
    }
}

impl<'a> NormalOp for XyRect<'a> {
    fn outward_normal(&self, _ray: &Ray, _t: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
}

impl<'a> NormalOp for XzRect<'a> {
    fn outward_normal(&self, _ray: &Ray, _t: f64) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
}

impl<'a> NormalOp for YzRect<'a> {
    fn outward_normal(&self, _ray: &Ray, _t: f64) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
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

impl<'a> UvOp for XzRect<'a> {
    fn get_u(&self, p: Vec3) -> f64 {
        (p.x - self.x0) / (self.x1 - self.x0)
    }
    fn get_v(&self, p: Vec3) -> f64 {
        (p.z - self.z0) / (self.z1 - self.z0)
    }
}

impl<'a> UvOp for YzRect<'a> {
    fn get_u(&self, p: Vec3) -> f64 {
        (p.z - self.z0) / (self.z1 - self.z0)
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

impl<'a> MaterialOp for XzRect<'a> {
    fn material(&self) -> &dyn Material {
        self.mp
    }
}

impl<'a> MaterialOp for YzRect<'a> {
    fn material(&self) -> &dyn Material {
        self.mp
    }
}
