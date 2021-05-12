use super::constants::*;
use super::vec3::*;

pub fn get_u(p: Vec3) -> f64 {
    let phi = (-p.z).atan2(p.x) + PI;
    phi / (2.0 * PI)
}
pub fn get_v(p: Vec3) -> f64 {
    let theta = (-p.y).acos();
    theta / PI
}
