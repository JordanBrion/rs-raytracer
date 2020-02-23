use super::vec3::*;
use super::sphere::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub trait Collider {
    type Element;

    fn hit(&self, element: &Self::Element) -> f32;
}

impl Collider for Ray {
    type Element = Sphere;

    fn hit(&self, element: &Self::Element) -> f32 {
        let rs = self.origin - element.center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * rs.dot(&self.direction);
        let c = rs.dot(&rs) - element.radius * element.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
