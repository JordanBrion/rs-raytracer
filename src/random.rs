extern crate rand;

use super::constants::*;
use super::vec3::*;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_limit(-1.0, 1.0);
        if p.squared_length() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_double_in_limit(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random_double(), random_double(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double_in_limit(0.0, 2.0 * PI);
    let z = random_double_in_limit(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}
