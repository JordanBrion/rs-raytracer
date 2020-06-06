use std::rc::Rc;

use super::aabb::*;
use super::bvh::*;
use super::cube::*;
use super::hittable::*;
use super::material::*;
use super::random::*;
use super::ray::*;
use super::rect::*;
use super::rotate::*;
use super::sphere::*;
use super::texture::*;
use super::translate::*;
use super::vec3::*;

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
        let t_box1 = Rc::new(Translate {
            ptr: r_box1.clone(),
            offset: Vec3::new(265.0, 0.0, 295.0),
        });

        let box2 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
        let t_box2 = Rc::new(Translate {
            ptr: r_box2.clone(),
            offset: Vec3::new(130.0, 0.0, 65.0),
        });

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
        let t_box1 = Rc::new(Translate {
            ptr: r_box1.clone(),
            offset: Vec3::new(265.0, 0.0, 295.0),
        });

        let box2 = Rc::new(Cube::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            materials.v_lambertians[1].clone(),
        ));
        let r_box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
        let t_box2 = Rc::new(Translate {
            ptr: r_box2.clone(),
            offset: Vec3::new(130.0, 0.0, 65.0),
        });

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
                Rc::new(ConstantMedium::new(
                    t_box1.clone(),
                    0.01,
                    Rc::new(SolidColor::new(0.0, 0.0, 0.0)),
                )),
                Rc::new(ConstantMedium::new(
                    t_box2.clone(),
                    0.01,
                    Rc::new(SolidColor::new(1.0, 1.0, 1.0)),
                )),
            ],
        }
    }

    pub fn final_scene_book2(materials: &Materials) -> HittableList {
        let mut v_boxes1: HittableList = Default::default();

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            let fi = i as f64;
            for j in 0..boxes_per_side {
                let fj = j as f64;
                let w = 100.0;
                let x0 = -1000.0 + fi * w;
                let y0 = 0.0;
                let z0 = -1000.0 + fj * w;
                let x1 = x0 + w;
                let y1 = random_double_in_limit(1.0, 101.0);
                let z1 = z0 + w;
                v_boxes1.add(Rc::new(Cube::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    materials.v_lambertians[0].clone(),
                )));
            }
        }

        let mut v_objects: std::vec::Vec<Rc<dyn Hittable>> = Default::default();

        v_objects.push(Rc::new(BVHNode::new(&mut v_boxes1, 0.0, 1.0)));

        v_objects.push(Rc::new(XzRect {
            x0: 123.0,
            x1: 423.0,
            z0: 147.0,
            z1: 412.0,
            k: 554.0,
            mp: materials.v_diffuse_lights[0].clone(),
        }));

        let center1 = Vec3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        v_objects.push(Rc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            materials.v_lambertians[1].clone(),
        )));

        v_objects.push(Rc::new(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            materials.v_dielectrics[0].clone(),
        )));
        v_objects.push(Rc::new(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            materials.v_metals[0].clone(),
        )));

        let boundary1 = Rc::new(Sphere::new(
            Vec3::new(360.0, 150.0, 145.0),
            70.0,
            materials.v_dielectrics[0].clone(),
        ));
        v_objects.push(boundary1.clone());
        v_objects.push(Rc::new(ConstantMedium::new(
            boundary1.clone(),
            0.2,
            Rc::new(SolidColor::new(0.2, 0.4, 0.9)),
        )));
        let boundary2 = Rc::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            5000.0,
            materials.v_dielectrics[0].clone(),
        ));
        v_objects.push(Rc::new(ConstantMedium::new(
            boundary2.clone(),
            0.0001,
            Rc::new(SolidColor::new(1.0, 1.0, 1.0)),
        )));

        v_objects.push(Rc::new(Sphere::new(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            materials.v_lambertians[2].clone(),
        )));

        v_objects.push(Rc::new(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            materials.v_lambertians[3].clone(),
        )));

        let mut boxes2: HittableList = Default::default();
        let ns = 1000;
        for _ in 0..ns {
            boxes2.add(Rc::new(Sphere::new(
                random_vec3_in_limit(0.0, 165.0),
                10.0,
                materials.v_lambertians[4].clone(),
            )));
        }

        v_objects.push(Rc::new(Translate {
            ptr: Rc::new(RotateY::new(
                Rc::new(BVHNode::new(&mut boxes2, 0.0, 1.0)),
                15.0,
            )),
            offset: Vec3::new(-100.0, 270.0, 395.0),
        }));

        HittableList {
            v_objects: v_objects,
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
