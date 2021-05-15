use super::aabb::*;
use super::bvh::*;
use super::cube::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::rectangle::*;
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
    v_xy_rects: HittableList<XyRect<'a>>,
    v_xz_rects: HittableList<XzRect<'a>>,
    v_yz_rects: HittableList<YzRect<'a>>,
    v_cubes: HittableList<Cube<'a>>,
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
        let v_hittable_xy_rects: Vec<&dyn Hittable> = self
            .v_xy_rects
            .0
            .iter()
            .map(|rectangle| rectangle as &dyn Hittable)
            .collect();
        let v_hittable_xz_rects: Vec<&dyn Hittable> = self
            .v_xz_rects
            .0
            .iter()
            .map(|rectangle| rectangle as &dyn Hittable)
            .collect();
        let v_hittable_yz_rects: Vec<&dyn Hittable> = self
            .v_yz_rects
            .0
            .iter()
            .map(|rectangle| rectangle as &dyn Hittable)
            .collect();
        let v_hittable_cubes: Vec<&dyn Hittable> = self
            .v_cubes
            .0
            .iter()
            .map(|cube| cube as &dyn Hittable)
            .collect();

        [
            v_hittable_spheres,
            v_hittable_moving_spheres,
            v_hittable_xy_rects,
            v_hittable_xz_rects,
            v_hittable_yz_rects,
            v_hittable_cubes,
        ]
        .concat()
    }

    pub fn new_cornell_box(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: Default::default(),
            v_moving_spheres: Default::default(),
            v_xy_rects: HittableList::from(vec![XyRect {
                mp: &materials.v_lambertians[1],
                x0: 0.0,
                x1: 555.0,
                y0: 0.0,
                y1: 555.0,
                k: 555.0,
            }]),
            v_xz_rects: HittableList::from(vec![
                XzRect {
                    mp: &materials.v_diffuse_lights[0],
                    x0: 213.0,
                    x1: 343.0,
                    z0: 227.0,
                    z1: 332.0,
                    k: 554.0,
                },
                XzRect {
                    mp: &materials.v_lambertians[1],
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                },
                XzRect {
                    mp: &materials.v_lambertians[1],
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                },
            ]),
            v_yz_rects: HittableList::from(vec![
                YzRect {
                    mp: &materials.v_lambertians[2],
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                },
                YzRect {
                    mp: &materials.v_lambertians[0],
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                },
            ]),
            v_cubes: HittableList(vec![
                Cube::new(
                    Vec3::new(130.0, 0.0, 65.0),
                    Vec3::new(295.0, 165.0, 230.0),
                    &materials.v_lambertians[1],
                ),
                Cube::new(
                    Vec3::new(265.0, 0.0, 295.0),
                    Vec3::new(430.0, 330.0, 460.0),
                    &materials.v_lambertians[1],
                ),
            ]),
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
