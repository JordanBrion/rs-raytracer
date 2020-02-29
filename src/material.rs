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
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Materials {
    pub fn new() -> Materials {
        Materials {
            v_metals: Default::default(),
            v_lambertians: vec![
                Lambertian {
                    albedo: Vec3::new(0.0, 0.0, 1.0),
                },
                Lambertian {
                    albedo: Vec3::new(1.0, 0.0, 0.0),
                },
            ],
            v_dielectrics: Default::default(),
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
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflected = ray.direction.reflect(record.normal);
        let d_dot_n = ray.direction.dot(&record.normal);
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let reflect_prob;
        let cosine;
        if d_dot_n > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * d_dot_n / ray.direction.length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -d_dot_n / ray.direction.length();
        }
        let refracted = if let Some(r) = ray.direction.refract(outward_normal, ni_over_nt) {
            reflect_prob = self.schlick(cosine);
            r
        } else {
            reflect_prob = 1.0;
            Vec3::new(0.0, 0.0, 0.0)
        };
        if random_number() < reflect_prob {
            *scattered = Ray::new(record.p, reflected);
        } else {
            *scattered = Ray::new(record.p, refracted);
        }
        true
    }
}
