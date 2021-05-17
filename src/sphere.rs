use super::aabb::*;
use super::constants::*;
use super::hittable::*;
use super::material::*;
use super::normal::*;
use super::ray::*;
use super::uv::*;
use super::vec3::*;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

pub struct MovingSphere<'a> {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f64, material: &'a dyn Material) -> Sphere<'a> {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl<'a> MovingSphere<'a> {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: &'a dyn Material,
    ) -> MovingSphere<'a> {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center0 + self.centers_vector() * self.percentage_time(time)
    }

    fn centers_vector(&self) -> Vec3 {
        self.center1 - self.center0
    }

    fn percentage_time(&self, time: f64) -> f64 {
        return (time - self.time0) / (self.time1 - self.time0);
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new(temp, self, ray));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::new(temp, self, ray));
            }
        }
        return None;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB {
            minimum: self.center - radius_vector,
            maximum: self.center + radius_vector,
        })
    }
}

impl<'a> Hittable for MovingSphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.squared_length();
        let half_b = oc.dot(&ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd_discriminant = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        Some(HitRecord::new(root, self, ray))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB {
            minimum: self.center(time0) - radius_vector,
            maximum: self.center(time0) + radius_vector,
        };
        let box1 = AABB {
            minimum: self.center(time1) - radius_vector,
            maximum: self.center(time1) + radius_vector,
        };
        Some(AABB::surrounding_box(box0, box1))
    }
}

impl<'a> NormalOp for Sphere<'a> {
    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3 {
        let hit_point = ray.point_at_parameter(t);
        (hit_point - self.center) / self.radius
    }
}

impl<'a> NormalOp for &Sphere<'a> {
    fn normal(&self, ray: &Ray, t: f64) -> (bool, Vec3) {
        (*self).normal(ray, t)
    }

    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3 {
        (*self).outward_normal(ray, t)
    }
}

impl<'a> NormalOp for MovingSphere<'a> {
    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3 {
        let hit_point = ray.point_at_parameter(t);
        (hit_point - self.center(ray.time)) / self.radius
    }
}

impl<'a> NormalOp for &MovingSphere<'a> {
    fn normal(&self, ray: &Ray, t: f64) -> (bool, Vec3) {
        (*self).normal(ray, t)
    }

    fn outward_normal(&self, ray: &Ray, t: f64) -> Vec3 {
        (*self).outward_normal(ray, t)
    }
}

impl<'a> MaterialOp for Sphere<'a> {
    fn material(&self) -> &dyn Material {
        self.material
    }
}

impl<'a> MaterialOp for &Sphere<'a> {
    fn material(&self) -> &dyn Material {
        (*self).material()
    }
}

impl<'a> MaterialOp for MovingSphere<'a> {
    fn material(&self) -> &dyn Material {
        self.material
    }
}

impl<'a> MaterialOp for &MovingSphere<'a> {
    fn material(&self) -> &dyn Material {
        (*self).material()
    }
}

impl<'a> UvOp for Sphere<'a> {
    fn get_u(&self, p: Vec3) -> f64 {
        get_sphere_u(p)
    }
    fn get_v(&self, p: Vec3) -> f64 {
        get_sphere_v(p)
    }
}

impl<'a> UvOp for MovingSphere<'a> {
    fn get_u(&self, p: Vec3) -> f64 {
        get_sphere_u(p)
    }
    fn get_v(&self, p: Vec3) -> f64 {
        get_sphere_v(p)
    }
}
