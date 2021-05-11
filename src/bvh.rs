use super::aabb::*;
use super::hittable::*;
use super::ray::*;

struct BvhNode<'a, 'b> {
    left: &'a dyn Hittable,
    right: &'b dyn Hittable,
    aabb: AABB,
}

impl<'a, 'b> Hittable for BvhNode<'a, 'b> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(_) = self.aabb.hit(ray, t_min, t_max) {
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
                (Some(_), None) => hit_left,
                (_, Some(_)) => hit_right,
                _ => None,
            };
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
