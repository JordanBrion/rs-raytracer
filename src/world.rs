use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub struct World<'a> {
    pub v_metals: std::vec::Vec<Metal>,
    pub v_lambertians: std::vec::Vec<Lambertian>,
    pub v_spheres: std::vec::Vec<Sphere<'a>>,
}

impl<'a> World<'a> {
    pub fn new<'b>() -> World<'b> {
        let mut world: World<'b> = Default::default();
        world.init_materials();
        world.init_spheres();
        world
    }

    fn init_materials(&mut self) {
        self.v_metals = vec![
            Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
            },
            Metal {
                albedo: Vec3::new(0.8, 0.8, 0.8),
            },
        ];
        self.v_lambertians = vec![
            Lambertian {
                albedo: Vec3::new(0.8, 0.3, 0.3),
            },
            Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            },
        ];
    }

    fn init_spheres(&'a mut self) {
        self.v_spheres = vec![
            Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &self.v_lambertians[0]),
            Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &self.v_lambertians[1]),
            Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &self.v_metals[0]),
            Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &self.v_metals[1]),
        ];
    }
}

impl<'a> Default for World<'a> {
    fn default() -> World<'a> {
        World {
            v_metals: Default::default(),
            v_lambertians: Default::default(),
            v_spheres: Default::default()
        }
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
        return closest_record;
    }
}
