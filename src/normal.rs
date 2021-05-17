use super::ray::*;
use super::vec3::*;

pub trait NormalOp {
    fn normal(&self, ray: &Ray, t: f64) -> (bool, Vec3) {
        make_facing_normal(ray, self.outward_normal(ray, t))
    }
    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3;
}

pub fn make_facing_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(&outward_normal) < 0.0f64;
    (
        front_face,
        match front_face {
            true => outward_normal,
            false => -outward_normal,
        },
    )
}
