mod Vec3;

use Vec3::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}