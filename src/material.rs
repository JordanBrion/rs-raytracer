extern crate libm;

use std::rc::Rc;

use super::hittable::*;
use super::random::*;
use super::ray::*;
use super::texture::*;
use super::vec3::*;

use libm::*;

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

pub struct Metal {
    pub albedo: Vec3,
    fuzz: f64,
}

pub struct Dielectric {
    ref_idx: f64,
}

pub struct Materials {
    pub v_lambertians: std::vec::Vec<Rc<Lambertian>>,
    pub v_metals: std::vec::Vec<Rc<Metal>>,
    pub v_dielectrics: std::vec::Vec<Rc<Dielectric>>,
    pub v_diffuse_lights: std::vec::Vec<Rc<DiffuseLight>>,
}

impl Metal {
    fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Materials {
    #[allow(dead_code)]
    pub fn new() -> Materials {
        Materials {
            v_lambertians: vec![
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.1, 0.2, 0.5)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.8, 0.8, 0.0)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(CheckerTexture {
                        even: Rc::new(SolidColor {
                            color_value: Vec3::new(0.2, 0.3, 0.1),
                        }),
                        odd: Rc::new(SolidColor {
                            color_value: Vec3::new(0.9, 0.9, 0.9),
                        }),
                    }),
                }),
            ],
            v_metals: vec![Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))],
            v_dielectrics: vec![Rc::new(Dielectric { ref_idx: 1.5 })],
            v_diffuse_lights: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_random() -> Materials {
        let mut materials = Materials {
            v_lambertians: vec![
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.5, 0.5, 0.5)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.4, 0.2, 0.1)),
                }),
            ],
            v_metals: vec![Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))],
            v_dielectrics: vec![Rc::new(Dielectric { ref_idx: 1.5 })],
            v_diffuse_lights: Default::default(),
        };
        for _ in -11..11 {
            for _ in -11..11 {
                materials.v_lambertians.push(Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new_random()),
                }));
                materials.v_metals.push(Rc::new(Metal::new(
                    random_color_in_limit(0.5, 1.0),
                    random_double_in_limit(0.0, 0.5),
                )));
            }
        }
        materials
    }

    #[allow(dead_code)]
    pub fn new_two_checkers() -> Materials {
        Materials {
            v_lambertians: vec![Rc::new(Lambertian {
                albedo: Rc::new(CheckerTexture {
                    odd: Rc::new(SolidColor::new(0.2, 0.3, 0.1)),
                    even: Rc::new(SolidColor::new(0.9, 0.9, 0.9)),
                }),
            })],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_two_perlins() -> Materials {
        Materials {
            v_lambertians: vec![Rc::new(Lambertian {
                albedo: Rc::new(NoiseTexture::new(2.0)),
            })],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_earth() -> Materials {
        Materials {
            v_lambertians: vec![Rc::new(Lambertian {
                albedo: Rc::new(ImageTexture::new(
                    "/home/jordanbrion/Documents/rust/rs-raytracer/resources/earthmap.jpg",
                )),
            })],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_light_source() -> Materials {
        Materials {
            v_lambertians: vec![Rc::new(Lambertian {
                albedo: Rc::new(NoiseTexture::new(4.0)),
            })],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: vec![Rc::new(DiffuseLight {
                emit: Box::new(SolidColor::new(4.0, 4.0, 4.0)),
            })],
        }
    }

    #[allow(dead_code)]
    pub fn new_empty_cornell_box() -> Materials {
        Materials {
            v_lambertians: vec![
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.65, 0.05, 0.05)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.73, 0.73, 0.73)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.12, 0.45, 0.15)),
                }),
            ],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: vec![Rc::new(DiffuseLight {
                emit: Box::new(SolidColor::new(15.0, 15.0, 15.0)),
            })],
        }
    }

    pub fn new_cornell_box() -> Materials {
        Materials {
            v_lambertians: vec![
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.65, 0.05, 0.05)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.73, 0.73, 0.73)),
                }),
                Rc::new(Lambertian {
                    albedo: Rc::new(SolidColor::new(0.12, 0.45, 0.15)),
                }),
            ],
            v_metals: Default::default(),
            v_dielectrics: Default::default(),
            v_diffuse_lights: vec![Rc::new(DiffuseLight {
                emit: Box::new(SolidColor::new(15.0, 15.0, 15.0)),
            })],
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

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.p, scatter_direction, ray_in.time);
        *attenuation = self.albedo.value(record.u, record.v, record.p);
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
        *scattered = Ray::new(
            record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            ray_in.time,
        );
        *attenuation = self.albedo;
        scattered.direction.dot(&record.normal) > 0.0
    }
}

impl Dielectric {
    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
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
        let cos_theta = fmin((-unit_direction).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(record.normal);
            *scattered = Ray::new(record.p, reflected, ray_in.time);
        } else if Self::schlick(cos_theta, etai_over_etat) > random_double() {
            let reflected = unit_direction.reflect(record.normal);
            *scattered = Ray::new(record.p, reflected, ray_in.time);
        } else {
            let refracted = Vec3::refract(unit_direction, record.normal, etai_over_etat);
            *scattered = Ray::new(record.p, refracted, ray_in.time);
        }
        true
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
