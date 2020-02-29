use super::hittable::*;
use super::random::*;
use super::ray::*;
use super::vec3::*;

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
    fuzz: f32,
}

pub struct Dielectric {
    ref_idx: f32
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
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Materials {
    pub fn new() -> Materials {
        Materials {
            v_metals: vec![
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0),
                Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0),
            ],
            v_lambertians: vec![
                Lambertian {
                    albedo: Vec3::new(0.1, 0.2, 0.5),
                },
                Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                },
            ],
            v_dielectrics: vec![
                Dielectric {
                    ref_idx: 1.5
                }
            ]
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
        let reflected = ray.direction.unit_vector().reflect(record.normal);
        *scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(&record.normal) > 0.0
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray.direction.reflect(record.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let outward_normal : Vec3;
        let ni_over_nt : f32;
        if ray.direction.dot(&record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
        } else {
                outward_normal = record.normal;
                ni_over_nt = 1.0 / self.ref_idx;
        }
        if let Some(refracted) = ray.direction.refract(outward_normal, ni_over_nt) {
            *scattered = Ray::new(record.p, refracted);
            true
        } else {
            *scattered = Ray::new(record.p, reflected);
            false
        }
    }
}
