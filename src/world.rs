use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub struct World<'a> {
    pub v_spheres: std::vec::Vec<Sphere<'a>>,
}

impl<'a> World<'a> {
    pub fn new(materials: &'a Materials, r: f32) -> World<'a> {
        World {
            v_spheres: vec![
                Sphere::new(Vec3::new(-r, 0.0, -1.0), r, &materials.v_lambertians[0]),
                Sphere::new(Vec3::new(r, 0.0, -1.0), r, &materials.v_lambertians[1]),
            ],
        }
    }
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for sphere in &self.v_spheres {
            if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }
}
