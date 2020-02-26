use super::hittable::*;
use super::random::*;
use super::ray::*;
use super::vec3::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
}

pub struct Materials {
    pub v_metals: std::vec::Vec<Metal>,
    pub v_lambertians: std::vec::Vec<Lambertian>,
}

impl Materials {
    pub fn new() -> Materials {
        Materials {
            v_metals: vec![
                Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                },
                Metal {
                    albedo: Vec3::new(0.8, 0.8, 0.8),
                },
            ],
            v_lambertians: vec![
                Lambertian {
                    albedo: Vec3::new(0.8, 0.3, 0.3),
                },
                Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                },
            ],
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = record.p + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.p, target - record.p);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), record.normal);
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(&record.normal) > 0.0
    }
}
