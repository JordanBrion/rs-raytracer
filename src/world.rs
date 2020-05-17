use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub struct World<'a> {
    pub v_spheres: std::vec::Vec<Sphere<'a>>,
    pub v_moving_spheres: std::vec::Vec<MovingSphere<'a>>,
}

impl<'a> World<'a> {
    #[allow(dead_code)]
    pub fn new(materials: &'a Materials) -> World<'a> {
        World {
            v_spheres: vec![
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
            ],
            v_moving_spheres: Default::default(),
        }
    }

    pub fn new_random(materials: &'a Materials) -> World<'a> {
        let mut world = World {
            v_spheres: vec![
                Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    &materials.v_lambertians[0],
                ),
                Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &materials.v_dielectrics[0]),
                Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &materials.v_lambertians[1]),
                Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &materials.v_metals[0]),
            ],
            v_moving_spheres: Default::default(),
        };
        let range_x: std::ops::Range<usize> = 0..22;
        let range_y: std::ops::Range<usize> = 0..22;
        let bias = 11;
        for a in range_y.clone() {
            for b in range_x.clone() {
                let choose_mat = random_double();
                let center = Vec3::new(
                    (a as i64 - bias) as f32 + 0.9 * random_double(),
                    0.2,
                    (b as i64 - bias) as f32 + 0.9 * random_double(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.v_moving_spheres.push(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, random_double_in_limit(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        if choose_mat < 0.8 {
                            &materials.v_lambertians[a * range_y.len() + b]
                        } else if choose_mat < 0.95 {
                            &materials.v_metals[a * range_y.len() + b]
                        } else {
                            &materials.v_dielectrics[0]
                        },
                    ));
                }
            }
        }
        world
    }
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for sphere in &self.v_spheres {
            if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        for sphere in &self.v_moving_spheres {
            if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }
}
