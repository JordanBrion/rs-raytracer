use super::color::*;
use super::vec3::*;

pub struct SolidColor {
    pub color_value: Color,
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.color_value
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
