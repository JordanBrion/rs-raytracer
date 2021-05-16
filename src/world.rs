use super::aabb::*;
use super::bvh::*;
use super::cube::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::rectangle::*;
use super::sphere::*;
use super::transform::*;
use super::vec3::*;

pub struct World<'a> {
    v_hittables: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> World<'a> {
    pub fn new_cornell_box(materials: &'a Materials) -> World<'a> {
        let cube_1 = Box::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            &materials.v_lambertians[1],
        ));
        let rotation_1 = Box::new(RotationY::new(cube_1, 15.0));
        let translation_1 = Box::new(Translate {
            offset: Vec3::new(265.0, 0.0, 295.0),
            ptr: rotation_1,
        });
        let cube_2 = Box::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            &materials.v_lambertians[1],
        ));
        let rotation_2 = Box::new(RotationY::new(cube_2, -18.0));
        let translation_2 = Box::new(Translate {
            offset: Vec3::new(130.0, 0.0, 65.0),
            ptr: rotation_2,
        });
        World {
            v_hittables: vec![
                Box::new(XyRect {
                    mp: &materials.v_lambertians[1],
                    x0: 0.0,
                    x1: 555.0,
                    y0: 0.0,
                    y1: 555.0,
                    k: 555.0,
                }),
                Box::new(XzRect {
                    mp: &materials.v_diffuse_lights[0],
                    x0: 213.0,
                    x1: 343.0,
                    z0: 227.0,
                    z1: 332.0,
                    k: 554.0,
                }),
                Box::new(XzRect {
                    mp: &materials.v_lambertians[1],
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                }),
                Box::new(XzRect {
                    mp: &materials.v_lambertians[1],
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                }),
                Box::new(YzRect {
                    mp: &materials.v_lambertians[2],
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                }),
                Box::new(YzRect {
                    mp: &materials.v_lambertians[0],
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                }),
                translation_1,
                translation_2,
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
