use super::ray::*;
use super::vec3::*;

pub trait NormalOp {
    fn normal(&self, ray: &Ray, t: f64) -> (bool, Vec3);
    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3;
}
