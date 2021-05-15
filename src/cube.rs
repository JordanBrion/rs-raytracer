use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::rectangle::*;
use super::vec3::*;
use super::world::*;

pub struct Cube<'a> {
    minimum: Vec3,
    maximum: Vec3,
    v_sides: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> Cube<'a> {
    pub fn new(minimum: Vec3, maximum: Vec3, material: &dyn Material) -> Cube {
        Cube {
            minimum: minimum,
            maximum: maximum,
            v_sides: vec![
                Box::new(XyRect {
                    mp: material,
                    x0: minimum.x,
                    x1: maximum.x,
                    y0: minimum.y,
                    y1: maximum.y,
                    k: minimum.z,
                }),
                Box::new(XyRect {
                    mp: material,
                    x0: minimum.x,
                    x1: maximum.x,
                    y0: minimum.y,
                    y1: maximum.y,
                    k: maximum.z,
                }),
                Box::new(XzRect {
                    mp: material,
                    x0: minimum.x,
                    x1: maximum.x,
                    z0: minimum.z,
                    z1: maximum.z,
                    k: minimum.y,
                }),
                Box::new(XzRect {
                    mp: material,
                    x0: minimum.x,
                    x1: maximum.x,
                    z0: minimum.z,
                    z1: maximum.z,
                    k: maximum.z,
                }),
                Box::new(YzRect {
                    mp: material,
                    y0: minimum.y,
                    y1: maximum.y,
                    z0: minimum.z,
                    z1: maximum.z,
                    k: minimum.x,
                }),
                Box::new(YzRect {
                    mp: material,
                    y0: minimum.y,
                    y1: maximum.y,
                    z0: minimum.z,
                    z1: maximum.z,
                    k: maximum.x,
                }),
            ],
        }
    }
}

impl<'a> Hittable for Cube<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for plane in &self.v_sides {
            if let Some(record) = plane.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.minimum,
            maximum: self.maximum,
        })
    }
}
