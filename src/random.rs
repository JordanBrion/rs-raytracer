extern crate rand;

use rand::Rng;
use super::vec3::*;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(random_number(), random_number(), random_number()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_number() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}