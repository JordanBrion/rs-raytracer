use super::sphere::*;
use super::vec3::*;
use super::hittable::*;
use super::ray::*;

pub struct World {
    pub v_spheres: std::vec::Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        let mut v_spheres = Vec::with_capacity(2);
        v_spheres.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
        v_spheres.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
        World {
            v_spheres: v_spheres,
        }
    }
}

impl Hittable for World {

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