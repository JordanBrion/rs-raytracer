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

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double_in_limit(min: f64, max: f64) -> f64 {
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

pub fn random_color() -> Vec3 {
    random_unit_vector()
}

pub fn random_color_in_limit(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_double_in_limit(min, max),
        random_double_in_limit(min, max),
        random_double_in_limit(min, max),
    )
}
