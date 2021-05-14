use super::aabb::*;
use super::material::*;
use super::normal::*;
use super::ray::*;
use super::uv::*;
use super::vec3::*;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    pub fn new<T>(t: f64, entity: &'a T, ray: &Ray) -> HitRecord<'a>
    where
        T: NormalOp + MaterialOp + UvOp,
    {
        let hit_point = ray.point_at_parameter(t);
        let outward_normal = entity.outward_normal(ray, t);
        let (front_facing, normal) = entity.normal(ray, t);
        HitRecord {
            t: t,
            p: hit_point,
            normal: normal,
            front_face: front_facing,
            material: entity.material(),
            u: entity.get_u(outward_normal),
            v: entity.get_v(outward_normal),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
