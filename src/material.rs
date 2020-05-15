extern crate libm;

use super::hittable::*;
use super::random::*;
use super::ray::*;
use super::vec3::*;
use libm::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
    fuzz: f32,
}

pub struct Dielectric {
    ref_idx: f32,
}

pub struct Materials {
    pub v_metals: std::vec::Vec<Metal>,
    pub v_lambertians: std::vec::Vec<Lambertian>,
    pub v_dielectrics: std::vec::Vec<Dielectric>,
}

impl Metal {
    fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Materials {
    pub fn new() -> Materials {
        Materials {
            v_metals: vec![
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0),
            ],
            v_lambertians: vec![
                Lambertian {
                    albedo: Vec3::new(0.1, 0.2, 0.5),
                },
                Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                },
            ],
            v_dielectrics: vec![Dielectric { ref_idx: 1.5 }],
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction.unit_vector().reflect(record.normal);
        *scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(&record.normal) > 0.0
    }
}

impl Dielectric {
    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = fmin((-unit_direction).dot(&record.normal) as f64, 1.0) as f32;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(record.normal);
            *scattered = Ray::new(record.p, reflected);
        } else {
            let refracted = Vec3::refract(unit_direction, record.normal, etai_over_etat);
            *scattered = Ray::new(record.p, refracted);
        }
        true
    }
}
