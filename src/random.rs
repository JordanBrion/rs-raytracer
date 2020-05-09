extern crate rand;

use super::vec3::*;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random_double(), random_double(), random_double())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            continue;
        } else {
            return p;
        }
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
