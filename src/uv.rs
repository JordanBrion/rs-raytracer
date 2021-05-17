use super::constants::*;
use super::vec3::*;

pub trait UvOp {
    fn get_u(&self, p: Vec3) -> f64;
    fn get_v(&self, p: Vec3) -> f64;
}

pub fn get_sphere_u(p: Vec3) -> f64 {
    let phi = (-p.z).atan2(p.x) + PI;
    phi / (2.0 * PI)
}

pub fn get_sphere_v(p: Vec3) -> f64 {
    let theta = (-p.y).acos();
    theta / PI
}
