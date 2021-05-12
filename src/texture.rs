use super::color::*;
use super::vec3::*;

pub struct SolidColor {
    pub color_value: Color,
}

pub struct CheckTexture<'a> {
    odd: &'a dyn Texture,
    even: &'a dyn Texture,
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.color_value
    }
}

impl<'a> Texture for CheckTexture<'a> {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let factor = 10.0;
        let sines = (factor * point.x).sin() + (factor * point.y).sin() * (factor * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct Textures {
    pub solid_colors: Vec<SolidColor>,
}

impl Textures {
    pub fn new() -> Textures {
        Textures {
            solid_colors: Default::default(),
        }
    }
}
