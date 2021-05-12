use super::aabb::*;
use super::bvh::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

struct HittableList<T>(Vec<T>);

impl<T> Default for HittableList<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> From<Vec<T>> for HittableList<T>
where
    T: Hittable,
{
    fn from(vector: Vec<T>) -> HittableList<T> {
        HittableList(vector)
    }
}

pub struct World<'a> {
    v_spheres: HittableList<Sphere<'a>>,
    v_moving_spheres: HittableList<MovingSphere<'a>>,
}

impl<'a> World<'a> {
    pub fn to_list_of_hittables(&self) -> Vec<&dyn Hittable> {
        let v_hittable_spheres: Vec<&dyn Hittable> = self
            .v_spheres
            .0
            .iter()
            .map(|sphere| sphere as &dyn Hittable)
            .collect();
        let v_hittable_moving_spheres: Vec<&dyn Hittable> = self
            .v_moving_spheres
            .0
            .iter()
            .map(|moving_sphere| moving_sphere as &dyn Hittable)
            .collect();
        [v_hittable_spheres, v_hittable_moving_spheres].concat()
    }

    #[allow(dead_code)]
    pub fn new(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: Default::default(),
            v_moving_spheres: Default::default(),
        }
    }
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let v_all_hittables = self.to_list_of_hittables();
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for sphere in v_all_hittables {
            if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let v_all_hittables = self.to_list_of_hittables();
        if v_all_hittables.is_empty() {
            return None;
        } else {
            let mut output_box: AABB = Default::default();
            let first_box = true;
            for hittable in v_all_hittables {
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
