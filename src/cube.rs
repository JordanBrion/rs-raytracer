use super::aabb::*;
use super::hittable::*;
use super::hittable_list::*;
use super::material::*;
use super::ray::*;
use super::rect::*;
use super::vec3::*;

use std::rc::*;

pub struct Cube {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, ptr: Rc<dyn Material>) -> Cube {
        let mut sides: HittableList = Default::default();
        sides.add(Rc::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.x(),
            mp: ptr.clone(),
        }));

        sides.add(Rc::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.x(),
            mp: ptr.clone(),
        }));

        sides.add(Rc::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.y(),
            mp: ptr.clone(),
        }));

        sides.add(Rc::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.y(),
            mp: ptr.clone(),
        }));

        sides.add(Rc::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p0.z(),
            mp: ptr.clone(),
        }));

        sides.add(Rc::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p1.z(),
            mp: ptr.clone(),
        }));

        Cube {
            box_min: p0,
            box_max: p1,
            sides: sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
