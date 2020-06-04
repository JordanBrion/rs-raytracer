use std::rc::Rc;

use super::aabb::*;
use super::cube::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::rect::*;
use super::sphere::*;
use super::vec3::*;
use super::rotate::*;
use super::translate::*;
use super::texture::*;

pub struct HittableList {
    pub v_objects: std::vec::Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            v_objects: Default::default(),
        }
    }
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 0.0, -1.0),
                    0.5,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    materials.v_lambertians[1].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(1.0, 0.0, -1.0),
                    0.5,
                    materials.v_metals[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    0.5,
                    materials.v_dielectrics[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-1.0, 0.0, -1.0),
                    -0.45,
                    materials.v_dielectrics[0].clone(),
                )),
            ],
        }
    }

    pub fn new_random(materials: &Materials) -> HittableList {
        let mut world = HittableList {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    materials.v_lambertians[2].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    1.0,
                    materials.v_dielectrics[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(-4.0, 1.0, 0.0),
                    1.0,
                    materials.v_lambertians[1].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(4.0, 1.0, 0.0),
                    1.0,
                    materials.v_metals[0].clone(),
                )),
            ],
        };
        let range_x: std::ops::Range<usize> = 0..22;
        let range_y: std::ops::Range<usize> = 0..22;
        let bias = 11;
        for a in range_y.clone() {
            for b in range_x.clone() {
                let choose_mat = random_double();
                let center = Vec3::new(
                    (a as i64 - bias) as f64 + 0.9 * random_double(),
                    0.2,
                    (b as i64 - bias) as f64 + 0.9 * random_double(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.v_objects.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, random_double_in_limit(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        if choose_mat < 0.8 {
                            materials.v_lambertians[a * range_y.len() + b].clone()
                        } else if choose_mat < 0.95 {
                            materials.v_metals[a * range_y.len() + b].clone()
                        } else {
                            materials.v_dielectrics[0].clone()
                        },
                    )));
                }
            }
        }
        world
    }

    #[allow(dead_code)]
    pub fn new_two_spheres(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 2.0, 0.0),
                    2.0,
                    materials.v_lambertians[0].clone(),
                )),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn new_earth(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![Rc::new(Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                2.0,
                materials.v_lambertians[0].clone(),
            ))],
        }
    }

    #[allow(dead_code)]
    pub fn new_light_source(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![
                Rc::new(Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 2.0, 0.0),
                    2.0,
                    materials.v_lambertians[0].clone(),
                )),
                Rc::new(Sphere::new(
                    Vec3::new(0.0, 7.0, 0.0),
                    2.0,
                    materials.v_diffuse_lights[0].clone(),
                )),
                Rc::new(XyRect {
                    x0: 3.0,
                    x1: 5.0,
                    y0: 1.0,
                    y1: 3.0,
                    k: -2.0,
                    mp: materials.v_diffuse_lights[0].clone(),
                }),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn new_empty_cornell_box(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![
                Rc::new(YzRect {
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                    mp: materials.v_lambertians[2].clone(),
                }),
                Rc::new(YzRect {
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                    mp: materials.v_lambertians[0].clone(),
                }),
                Rc::new(XzRect {
                    x0: 213.0,
                    x1: 343.0,
                    z0: 227.0,
                    z1: 332.0,
                    k: 554.0,
                    mp: materials.v_diffuse_lights[0].clone(),
                }),
                Rc::new(XzRect {
                    mp: materials.v_lambertians[1].clone(),
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                }),
                Rc::new(XzRect {
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                    mp: materials.v_lambertians[1].clone(),
                }),
                Rc::new(XyRect {
                    x0: 0.0,
                    x1: 555.0,
                    y0: 0.0,
                    y1: 555.0,
                    k: 555.0,
                    mp: materials.v_lambertians[1].clone(),
                }),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn new_cornell_box(materials: &Materials) -> HittableList {
        HittableList {
            v_objects: vec![
                Rc::new(FlipFace {
                    ptr: Rc::new(YzRect {
                        y0: 0.0,
                        y1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[2].clone(),
                    }),
                }),
                Rc::new(YzRect {
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                    mp: materials.v_lambertians[0].clone(),
                }),
                Rc::new(XzRect {
                    x0: 213.0,
                    x1: 343.0,
                    z0: 227.0,
                    z1: 332.0,
                    k: 554.0,
                    mp: materials.v_diffuse_lights[0].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XzRect {
                        mp: materials.v_lambertians[1].clone(),
                        x0: 0.0,
                        x1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 0.0,
                    }),
                }),
                Rc::new(XzRect {
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                    mp: materials.v_lambertians[1].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XyRect {
                        x0: 0.0,
                        x1: 555.0,
                        y0: 0.0,
                        y1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[1].clone(),
                    }),
                }),
                Rc::new(Cube::new(
                    Vec3::new(130.0, 0.0, 65.0),
                    Vec3::new(295.0, 165.0, 230.0),
                    materials.v_lambertians[1].clone(),
                )),
                Rc::new(Cube::new(
                    Vec3::new(265.0, 0.0, 295.0),
                    Vec3::new(430.0, 330.0, 460.0),
                    materials.v_lambertians[1].clone(),
                )),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn new_rotated_cornell_box(materials: &Materials) -> HittableList {
        let box1 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box1 = Rc::new(RotateY::new(box1.clone(), 15.0));
        let t_box1 = Rc::new(Translate { ptr: r_box1.clone(), offset: Vec3::new(265.0, 0.0, 295.0) });

        let box2 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
        let t_box2 = Rc::new(Translate { ptr: r_box2.clone(), offset: Vec3::new(130.0, 0.0, 65.0) });

        HittableList {
            v_objects: vec![
                Rc::new(FlipFace {
                    ptr: Rc::new(YzRect {
                        y0: 0.0,
                        y1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[2].clone(),
                    }),
                }),
                Rc::new(YzRect {
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                    mp: materials.v_lambertians[0].clone(),
                }),
                Rc::new(XzRect {
                    x0: 213.0,
                    x1: 343.0,
                    z0: 227.0,
                    z1: 332.0,
                    k: 554.0,
                    mp: materials.v_diffuse_lights[0].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XzRect {
                        mp: materials.v_lambertians[1].clone(),
                        x0: 0.0,
                        x1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 0.0,
                    }),
                }),
                Rc::new(XzRect {
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 555.0,
                    mp: materials.v_lambertians[1].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XyRect {
                        x0: 0.0,
                        x1: 555.0,
                        y0: 0.0,
                        y1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[1].clone(),
                    }),
                }),
                t_box1,
                t_box2,
            ],
        }
    }

    pub fn new_smoked_cornell_box(materials: &Materials) -> HittableList {
        let box1 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box1 = Rc::new(RotateY::new(box1.clone(), 15.0));
        let t_box1 = Rc::new(Translate { ptr: r_box1.clone(), offset: Vec3::new(265.0, 0.0, 295.0) });

        let box2 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
        let t_box2 = Rc::new(Translate { ptr: r_box2.clone(), offset: Vec3::new(130.0, 0.0, 65.0) });

        HittableList {
            v_objects: vec![
                Rc::new(FlipFace {
                    ptr: Rc::new(YzRect {
                        y0: 0.0,
                        y1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[2].clone(),
                    }),
                }),
                Rc::new(YzRect {
                    y0: 0.0,
                    y1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                    mp: materials.v_lambertians[0].clone(),
                }),
                Rc::new(XzRect {
                    x0: 113.0,
                    x1: 443.0,
                    z0: 127.0,
                    z1: 432.0,
                    k: 554.0,
                    mp: materials.v_diffuse_lights[0].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XzRect {
                        mp: materials.v_lambertians[1].clone(),
                        x0: 0.0,
                        x1: 555.0,
                        z0: 0.0,
                        z1: 555.0,
                        k: 555.0,
                    }),
                }),
                Rc::new(XzRect {
                    x0: 0.0,
                    x1: 555.0,
                    z0: 0.0,
                    z1: 555.0,
                    k: 0.0,
                    mp: materials.v_lambertians[1].clone(),
                }),
                Rc::new(FlipFace {
                    ptr: Rc::new(XyRect {
                        x0: 0.0,
                        x1: 555.0,
                        y0: 0.0,
                        y1: 555.0,
                        k: 555.0,
                        mp: materials.v_lambertians[1].clone(),
                    }),
                }),
                Rc::new(ConstantMedium::new(t_box1.clone(), 0.01, Rc::new(SolidColor::new(0.0, 0.0, 0.0)))),
                Rc::new(ConstantMedium::new(t_box2.clone(), 0.01, Rc::new(SolidColor::new(1.0, 1.0, 1.0)))),
            ],
        }
    }

    pub fn add(&mut self, hittable: std::rc::Rc<dyn Hittable>) {
        self.v_objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for object in &self.v_objects {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                closest_record = Some(record);
            }
        }
        return closest_record;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.v_objects.is_empty() {
            None
        } else {
            let mut output_box = Some(Default::default());
            let mut first_box = true;
            for object in &self.v_objects {
                if let Some(temp_box) = (*object).bounding_box(t0, t1) {
                    output_box = if first_box {
                        Some(temp_box)
                    } else {
                        Some(AABB::surrounding_box(temp_box, output_box.unwrap()))
                    };
                    first_box = false;
                } else {
                    return None;
                }
            }
            output_box
        }
    }
}
