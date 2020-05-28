extern crate image;
extern crate num;

use super::constants::*;
use super::noise::*;
use super::random::*;
use super::vec3::*;

use std::rc::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new(x: f64, y: f64, z: f64) -> SolidColor {
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
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
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
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            perlin: Perlin::new(),
            scale: scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.perlin.turb(p, 7)).sin())
    }
}

pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = (p.z()).atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

pub struct ImageTexture {
    image: image::RgbImage,
}

impl ImageTexture {
    pub fn new(file_name: &str) -> ImageTexture {
        let img = image::open(file_name).unwrap();
        ImageTexture {
            image: img.to_rgb(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let (width, height) = self.image.dimensions();
        let uu = num::clamp(u, 0.0, 1.0);
        let vv = 1.0 - num::clamp(v, 0.0, 1.0);
        let i = (uu * width as f64) as u32;
        let j = (vv * height as f64) as u32;
        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        Vec3::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
