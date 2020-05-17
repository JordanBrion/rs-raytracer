use super::vec3::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new(Default::default(), Default::default(), Default::default())
    }
}
