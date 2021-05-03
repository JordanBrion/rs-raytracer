use super::material::*;
use super::ray::*;
use super::sphere::*;
use super::vec3::*;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new<T>(t: f64, sphere: &'a T, ray: &Ray) -> HitRecord<'a>
    where
        T: NormalOp + MaterialOp,
    {
        let hit_point = ray.point_at_parameter(t);
        let (front_facing, normal) = sphere.normal(ray, t);
        HitRecord {
            t: t,
            p: hit_point,
            normal: normal,
            front_face: front_facing,
            material: sphere.material(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
