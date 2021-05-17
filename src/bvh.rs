use std::{cmp::Ordering, rc::Rc};

use super::aabb::*;
use super::hittable::*;
use super::random::*;
use super::ray::*;
use super::vec3::*;

enum BvhChild<'a> {
    Parent(Rc<BvhNode<'a>>),
    Leaf(Rc<dyn Hittable + 'a>),
}

pub struct BvhNode<'a> {
    left: BvhChild<'a>,
    right: BvhChild<'a>,
    aabb: AABB,
}

impl<'a> BvhNode<'a> {
    pub fn new(v_hittables: &[Rc<dyn Hittable + 'a>], time0: f64, time1: f64) -> BvhNode<'a> {
        let axis = random_integer_in_limit(0, 2);
        let comparator = |hittable0: Rc<dyn Hittable>, hittable1: Rc<dyn Hittable>| {
            return match (
                hittable0.bounding_box(0.0, 0.0),
                hittable1.bounding_box(0.0, 0.0),
            ) {
                (Some(aabb0), Some(aabb1)) => aabb0.minimum[axis]
                    .partial_cmp(&aabb1.minimum[axis])
                    .unwrap_or(Ordering::Greater),
                (Some(aabb0), None) => aabb0.minimum[axis]
                    .partial_cmp(&Vec3::default()[axis])
                    .unwrap_or(Ordering::Greater),
                (None, Some(aabb1)) => Vec3::default()[axis]
                    .partial_cmp(&aabb1.minimum[axis])
                    .unwrap_or(Ordering::Greater),
                _ => Ordering::Greater,
            };
        };
        let object_span = v_hittables.len();
        let (left, right) = match object_span {
            1 => (
                BvhChild::Leaf(v_hittables[0].clone()),
                BvhChild::Leaf(v_hittables[0].clone()),
            ),
            2 => {
                if comparator(v_hittables[0].clone(), v_hittables[1].clone()) == Ordering::Less {
                    (
                        BvhChild::Leaf(v_hittables[0].clone()),
                        BvhChild::Leaf(v_hittables[1].clone()),
                    )
                } else {
                    (
                        BvhChild::Leaf(v_hittables[1].clone()),
                        BvhChild::Leaf(v_hittables[0].clone()),
                    )
                }
            }
            _ => {
                let mid = v_hittables.len() / 2;
                let (left_array, right_array) = v_hittables.split_at(mid);
                (
                    BvhChild::Parent(Rc::new(BvhNode::new(left_array, time0, time1))),
                    BvhChild::Parent(Rc::new(BvhNode::new(right_array, time0, time1))),
                )
            }
        };
        let aabb0 = left.bounding_box(time0, time1);
        let aabb1 = right.bounding_box(time0, time1);
        BvhNode {
            left: left,
            right: right,
            aabb: match (aabb0, aabb1) {
                (Some(aabb0), Some(aabb1)) => AABB::surrounding_box(aabb0, aabb1),
                (Some(aabb0), _) => AABB::surrounding_box(aabb0, Default::default()),
                _ => Default::default(),
            },
        }
    }
}

impl<'a> Hittable for BvhNode<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = self.right.hit(
                ray,
                t_min,
                match hit_left {
                    Some(_) => hit_left.as_ref().unwrap().t,
                    _ => t_max,
                },
            );
            return match (&hit_left, &hit_right) {
                (_, Some(_)) => hit_right,
                (Some(_), _) => hit_left,
                _ => None,
            };
        } else {
            return None;
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}

impl<'a> Hittable for BvhChild<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return match self {
            BvhChild::Parent(node) => node.hit(ray, t_min, t_max),
            BvhChild::Leaf(node) => node.hit(ray, t_min, t_max),
        };
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        return match self {
            BvhChild::Parent(node) => node.bounding_box(time0, time1),
            BvhChild::Leaf(node) => node.bounding_box(time0, time1),
        };
    }
}
