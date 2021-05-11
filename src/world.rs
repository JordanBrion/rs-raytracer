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
    #[allow(dead_code)]
    pub fn new(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: HittableList::from(vec![
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &materials.v_lambertians[0]),
                Sphere::new(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    &materials.v_lambertians[1],
                ),
                Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &materials.v_metals[0]),
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &materials.v_dielectrics[0]),
                Sphere::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    -0.45,
                    &materials.v_dielectrics[0],
                ),
            ]),
            v_moving_spheres: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_random(materials: &'a Materials) -> World<'a> {
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
                    world.v_spheres.0.push(Sphere::new(
                        center,
                        0.2,
                        if choose_mat < 0.8 {
                            &materials.v_lambertians[a * end + b]
                        } else if choose_mat < 0.95 {
                            &materials.v_metals[a * end + b]
                        } else {
                            &materials.v_dielectrics[0]
                        },
                    ));
                }
            }
        }
        world
    }

    pub fn new_random_with_moving_spheres(materials: &'a Materials) -> World {
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
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let v_all_hittables = [v_hittable_spheres, v_hittable_moving_spheres].concat();
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
}
