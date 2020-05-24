use super::constants::*;
use super::noise::*;
use super::random::*;
use super::vec3::*;

use std::rc::*;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new(x: f32, y: f32, z: f32) -> SolidColor {
        SolidColor {
            color_value: Vec3::new(x, y, z),
        }
    }

    pub fn new_random() -> SolidColor {
        SolidColor {
            color_value: random_color() * random_color(),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    perlin: Perlin,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            perlin: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.perlin.noise(p)
    }
}

pub fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let phi = (p.z()).atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}
