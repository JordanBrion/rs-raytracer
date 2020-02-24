use super::hittable::*;
use super::ray::*;
use super::vec3::*;
use super::random::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3
}

pub struct Metal {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = record.p + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.p, target - record.p);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), record.normal);
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(&record.normal) > 0.0
    }
}
