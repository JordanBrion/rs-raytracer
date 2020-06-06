use std::rc::Rc;

use super::aabb::*;
use super::hittable::*;
use super::hittable_list::*;
use super::random::*;
use super::ray::*;
use super::sphere::*;

use std::cmp::Ordering;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    aabb: AABB,
}

fn make_boundaries(
    v_objects: &[Rc<dyn Hittable>],
    axis: usize,
) -> (Option<Rc<dyn Hittable>>, Option<Rc<dyn Hittable>>) {
    let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
        let box0 = a.bounding_box(0.0, 0.0);
        let box1 = b.bounding_box(0.0, 0.0);
        return match (box0, box1) {
            (Some(aabb0), Some(aabb1)) => aabb0.min.e[axis].partial_cmp(&aabb1.min.e[axis]),
            _ => None,
        };
    };
    return match v_objects.len() {
        1 => (Some(v_objects[0].clone()), Some(v_objects[0].clone())),
        2 => {
            return match comparator(&v_objects[0], &v_objects[1]) {
                Some(Ordering::Less) => (Some(v_objects[0].clone()), Some(v_objects[1].clone())),
                Some(Ordering::Greater) => (Some(v_objects[1].clone()), Some(v_objects[0].clone())),
                _ => (None, None),
            }
        }
        _ => (None, None),
    };
}

impl BVHNode {
    pub fn new(world: &mut HittableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::new_a(&mut world.v_objects[0..], time0, time1)
    }

    fn new_a(v_objects: &mut [Rc<dyn Hittable>], time0: f64, time1: f64) -> BVHNode {
        let axis = random_int_in_limit(0, 2) as usize;
        let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            let box0 = a.bounding_box(0.0, 0.0);
            let box1 = b.bounding_box(0.0, 0.0);
            return match (box0, box1) {
                (Some(aabb0), Some(aabb1)) => {
                    aabb0.min.e[axis].partial_cmp(&aabb1.min.e[axis]).unwrap()
                }
                _ => Ordering::Equal,
            };
        };
        let boundaries = make_boundaries(v_objects, axis);
        return match boundaries {
            (Some(left), Some(right)) => BVHNode {
                left: left.clone(),
                right: right.clone(),
                aabb: match (
                    left.bounding_box(time0, time1),
                    right.bounding_box(time0, time1),
                ) {
                    (Some(aabb0), Some(aabb1)) => AABB::surrounding_box(aabb0, aabb1),
                    (_, Some(aabb)) => aabb,
                    (Some(aabb), _) => aabb,
                    _ => Default::default(),
                },
            },
            _ => {
                v_objects.sort_by(comparator);
                let mid = v_objects.len() / 2;
                let left = Rc::new(BVHNode::new_a(&mut v_objects[0..mid], time0, time1));
                let right = Rc::new(BVHNode::new_a(&mut v_objects[mid..], time0, time1));

                BVHNode {
                    left: left.clone(),
                    right: right.clone(),
                    aabb: match (
                        left.bounding_box(time0, time1),
                        right.bounding_box(time0, time1),
                    ) {
                        (Some(aabb0), Some(aabb1)) => AABB::surrounding_box(aabb0, aabb1),
                        (_, Some(aabb)) => aabb,
                        (Some(aabb), _) => aabb,
                        _ => Default::default(),
                    },
                }
            }
        };
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            None
        } else if let Some(record) = self.left.hit(ray, t_min, t_max) {
            Some(record)
        } else if let Some(record) = self.right.hit(ray, t_min, t_max) {
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
