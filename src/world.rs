use std::rc::Rc;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub struct World {
    pub v_objects: std::vec::Vec<Rc<dyn Hittable>>,
}

impl World {
    #[allow(dead_code)]
    pub fn new(materials: &Materials) -> World {
        World {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 0.0, -1.0),
                    0.5,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    materials.v_lambertians[1].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(1.0, 0.0, -1.0),
                    0.5,
                    materials.v_metals[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    0.5,
                    materials.v_dielectrics[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    -0.45,
                    materials.v_dielectrics[0].clone(),
                )),
            ],
        }
    }

    pub fn new_random(materials: &Materials) -> World {
        let mut world = World {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    1.0,
                    materials.v_dielectrics[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-4.0, 1.0, 0.0),
                    1.0,
                    materials.v_lambertians[1].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(4.0, 1.0, 0.0),
                    1.0,
                    materials.v_metals[0].clone(),
                )),
            ],
        };
        let range_x: std::ops::Range<usize> = 0..22;
        let range_y: std::ops::Range<usize> = 0..22;
        let bias = 11;
        for a in range_y.clone() {
            for b in range_x.clone() {
                let choose_mat = random_double();
                let center = Vec3::new(
                    (a as i64 - bias) as f32 + 0.9 * random_double(),
                    0.2,
                    (b as i64 - bias) as f32 + 0.9 * random_double(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.v_objects.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, random_double_in_limit(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        if choose_mat < 0.8 {
                            materials.v_lambertians[a * range_y.len() + b].clone()
                        } else if choose_mat < 0.95 {
                            materials.v_metals[a * range_y.len() + b].clone()
                        } else {
                            materials.v_dielectrics[0].clone()
                        },
                    )));
                }
            }
        }
        world
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for object in &self.v_objects {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.v_objects.is_empty() {
            None
        } else {
            let mut output_box = Some(Default::default());
            let mut first_box = true;
            for object in &self.v_objects {
                if let Some(temp_box) = (*object).bounding_box(t0, t1) {
                    output_box = if first_box {
                        Some(temp_box)
                    } else {
                        Some(AABB::surrounding_box(temp_box, output_box.unwrap()))
                    };
                    first_box = false;
                } else {
                    return None;
                }
            }
            output_box
        }
    }
}
