use std::rc::Rc;

use super::aabb::*;
use super::bvh::*;
use super::cube::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::random::*;
use super::ray::*;
use super::rectangle::*;
use super::sphere::*;
use super::transform::*;
use super::vec3::*;
use super::volume::*;

pub struct World<'a> {
    v_hittables: Vec<Rc<dyn Hittable + 'a>>,
}

impl<'a> World<'a> {
    pub fn new_final_scene(materials: &'a Materials) -> World<'a> {
        let iterations = 20;
        let mut v_hittables_1: Vec<Rc<dyn Hittable + 'a>> =
            Vec::with_capacity(iterations * iterations);
        for i in 0..iterations {
            let f_i = i as f64;
            for j in 0..iterations {
                let f_j = j as f64;
                let w = 100.0;
                let x0 = -1000.0 + f_i * w;
                let z0 = -1000.0 + f_j * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_double_in_limit(1.0, 101.0);
                let z1 = z0 + w;
                v_hittables_1.push(Rc::new(Cube::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    &materials.v_lambertians[0],
                )));
            }
        }
        let center_1 = Vec3::new(400.0, 400.0, 200.0);
        let center_2 = center_1 + Vec3::new(30.0, 0.0, 0.0);
        let mut v_hittables_2: Vec<Rc<dyn Hittable + 'a>> = Vec::with_capacity(1000);
        for _ in 0..1000 {
            v_hittables_2.push(Rc::new(Sphere::new(
                Vec3::random_in_limit(0.0, 165.0),
                10.0,
                &materials.v_lambertians[4],
            )));
        }

        World {
            v_hittables: vec![
                Rc::new(BvhNode::new(&v_hittables_1, 0.0, 1.0)),
                Rc::new(Translate {
                    offset: Vec3::new(-100.0, 270.0, 395.0),
                    ptr: Rc::new(RotationY::new(
                        Rc::new(BvhNode::new(&v_hittables_2, 0.0, 1.0)),
                        15.0,
                    )),
                }),
                Rc::new(XzRect {
                    x0: 123.0,
                    x1: 423.0,
                    z0: 147.0,
                    z1: 412.0,
                    k: 554.0,
                    mp: &materials.v_diffuse_lights[0],
                }),
                Rc::new(MovingSphere::new(
                    center_1,
                    center_2,
                    0.0,
                    1.0,
                    50.0,
                    &materials.v_lambertians[1],
                )),
                Rc::new(Sphere::new(
                    Vec3::new(260.0, 150.0, 45.0),
                    50.0,
                    &materials.v_dielectrics[0],
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 150.0, 145.0),
                    50.0,
                    &materials.v_metals[0],
                )),
                Rc::new(ConstantMedium::new(
                    Rc::new(Sphere::new(
                        Vec3::new(360.0, 150.0, 145.0),
                        70.0,
                        &materials.v_dielectrics[0],
                    )),
                    0.2,
                    &materials.v_isotropics[0],
                )),
                Rc::new(ConstantMedium::new(
                    Rc::new(Sphere::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        5000.0,
                        &materials.v_dielectrics[0],
                    )),
                    0.0001,
                    &materials.v_isotropics[1],
                )),
                Rc::new(Sphere::new(
                    Vec3::new(400.0, 200.0, 400.0),
                    100.0,
                    &materials.v_lambertians[2],
                )),
                Rc::new(Sphere::new(
                    Vec3::new(220.0, 280.0, 300.0),
                    80.0,
                    &materials.v_lambertians[3],
                )),
            ],
        }
    }
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for sphere in &self.v_hittables {
            if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.v_hittables.is_empty() {
            return None;
        } else {
            let mut output_box: AABB = Default::default();
            let first_box = true;
            for hittable in &self.v_hittables {
                if let Some(temp_box) = hittable.bounding_box(time0, time1) {
                    output_box = if first_box {
                        temp_box
                    } else {
                        AABB::surrounding_box(output_box, temp_box)
                    };
                } else {
                    return None;
                }
            }
            return Some(output_box);
        }
    }
}
