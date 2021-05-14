use super::aabb::*;
use super::bvh::*;
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
        [
            v_hittable_spheres,
            v_hittable_moving_spheres,
            v_hittable_xy_rects,
        ]
        .concat()
    }

    #[allow(dead_code)]
    pub fn new(materials: &'a Materials) -> World<'a> {
        let mut world = World {
            v_spheres: HittableList::from(vec![
                Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    &materials.v_lambertians[0],
                ),
                Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &materials.v_dielectrics[0]),
                Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &materials.v_lambertians[1]),
                Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &materials.v_metals[0]),
            ]),
            v_moving_spheres: Default::default(),
            v_xy_rects: Default::default(),
        };
        let start = 0;
        let end = 22;
        let bias = 11;
        for a in start..end {
            for b in start..end {
                let choose_mat = random_double();
                let center = Vec3::new(
                    (a as i64 - bias) as f64 + 0.9 * random_double(),
                    0.2,
                    (b as i64 - bias) as f64 + 0.9 * random_double(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        world.v_moving_spheres.0.push(MovingSphere::new(
                            center,
                            center + Vec3::new(0.0, random_double_in_limit(0.0, 0.5), 0.0),
                            0.0,
                            1.0,
                            0.2,
                            &materials.v_lambertians[a * end + b],
                        ));
                    } else if choose_mat < 0.95 {
                        world.v_spheres.0.push(Sphere::new(
                            center,
                            0.2,
                            &materials.v_metals[a * end + b],
                        ));
                    } else {
                        world.v_spheres.0.push(Sphere::new(
                            center,
                            0.2,
                            &materials.v_dielectrics[0],
                        ));
                    }
                }
            }
        }
        world
    }

    #[allow(dead_code)]
    pub fn new_two_spheres(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: HittableList::from(vec![
                Sphere::new(
                    Vec3::new(0.0, -10.0, 0.0),
                    10.0,
                    &materials.v_lambertians[0],
                ),
                Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, &materials.v_lambertians[0]),
            ]),
            v_moving_spheres: Default::default(),
            v_xy_rects: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_two_perlin_spheres(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: HittableList::from(vec![
                Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    &materials.v_lambertians[0],
                ),
                Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, &materials.v_lambertians[0]),
            ]),
            v_moving_spheres: Default::default(),
            v_xy_rects: Default::default(),
        }
    }

    pub fn new_simple_light(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: HittableList::from(vec![
                Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    &materials.v_lambertians[0],
                ),
                Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, &materials.v_lambertians[0]),
            ]),
            v_moving_spheres: Default::default(),
            v_xy_rects: HittableList::from(vec![XyRect {
                mp: &materials.v_diffuse_lights[0],
                x0: 3.0,
                x1: 5.0,
                y0: 1.0,
                y1: 3.0,
                k: -2.0,
            }]),
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
