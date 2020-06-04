use std::rc::Rc;

use super::aabb::*;
use super::constants::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::rect::*;
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
        let (u, v) = get_sphere_uv((hit_point - sphere.center) / sphere.radius);
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
        let (u, v) = get_sphere_uv((hit_point - sphere.center(t)) / sphere.radius);
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

    pub fn new_rect_xy(t: f64, rect: &XyRect, ray: &Ray, x: f64, y: f64) -> HitRecord {
        let u = (x - rect.x0) / (rect.x1 - rect.x0);
        let v = (y - rect.y0) / (rect.y1 - rect.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            t: t,
            p: ray.point_at_parameter(t),
            normal: final_normal,
            u: u,
            v: v,
            front_face: front_face,
            material: rect.mp.clone(),
        }
    }

    pub fn new_rect_xz(t: f64, rect: &XzRect, ray: &Ray, x: f64, z: f64) -> HitRecord {
        let u = (x - rect.x0) / (rect.x1 - rect.x0);
        let v = (z - rect.z0) / (rect.z1 - rect.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            t: t,
            p: ray.point_at_parameter(t),
            normal: final_normal,
            u: u,
            v: v,
            front_face: front_face,
            material: rect.mp.clone(),
        }
    }

    pub fn new_rect_yz(t: f64, rect: &YzRect, ray: &Ray, y: f64, z: f64) -> HitRecord {
        let u = (y - rect.y0) / (rect.y1 - rect.y0);
        let v = (z - rect.z0) / (rect.z1 - rect.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let final_normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            t: t,
            p: ray.point_at_parameter(t),
            normal: final_normal,
            u: u,
            v: v,
            front_face: front_face,
            material: rect.mp.clone(),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub struct FlipFace {
    pub ptr: std::rc::Rc<dyn Hittable>,
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut record) = self.ptr.hit(ray, t_min, t_max) {
            record.front_face = !record.front_face;
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.ptr.bounding_box(t0, t1)
    }
}

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Rc::new(Isotropic { albedo: a }),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;
        if let Some(mut rec1) = self.boundary.hit(ray, -INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.001, INFINITY) {
                if debugging {
                    println!("t0={0};t1={1};", rec1.t, rec2.t);
                }
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_double().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                } else {
                    let t = rec1.t + hit_distance / ray_length;
                    let p = ray.point_at_parameter(t);
                    if debugging {
                        println!("hit_distance = {}", hit_distance);
                        println!("t = {}", t);
                        println!("p = {}", p);
                    }
                    return Some(HitRecord {
                        t: t,
                        p: p,
                        normal: Vec3::new(1.0, 0.0, 0.0),
                        u: 0.0,
                        v: 0.0,
                        front_face: true,
                        material: self.phase_function.clone(),
                    });
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
