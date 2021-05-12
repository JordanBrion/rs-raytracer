use super::color::*;
use super::random::*;
use super::vec3::*;

pub struct SolidColor {
    pub color_value: Color,
}

pub struct CheckerTexture {
    odd: SolidColor,
    even: SolidColor,
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        self.color_value
    }
}

impl<'a> Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let factor = 10.0;
        let sines = (factor * point.x).sin() * (factor * point.y).sin() * (factor * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct Textures {
    pub v_solid_colors: Vec<SolidColor>,
    pub v_checker_textures: Vec<CheckerTexture>,
}

impl<'a> Textures {
    pub fn new() -> Textures {
        let mut textures = Textures {
            v_solid_colors: vec![
                SolidColor {
                    color_value: Vec3::new(0.5, 0.5, 0.5),
                },
                SolidColor {
                    color_value: Vec3::new(0.4, 0.2, 0.1),
                },
            ],
            v_checker_textures: vec![CheckerTexture {
                even: SolidColor {
                    color_value: Vec3::new(0.2, 0.3, 0.1),
                },
                odd: SolidColor {
                    color_value: Vec3::new(0.9, 0.9, 0.9),
                },
            }],
        };
        for _ in -11..11 {
            textures.v_solid_colors.push(SolidColor {
                color_value: random_color() * random_color(),
            })
        }
        textures
    }
}
