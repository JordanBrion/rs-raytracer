use crate::color;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel};
use num::clamp;

use super::color::*;
use super::perlin::*;
use super::random::*;
use super::vec3::*;

use std::cmp::*;

pub struct SolidColor {
    pub color_value: Color,
}

pub struct CheckerTexture {
    odd: SolidColor,
    even: SolidColor,
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: scale,
        }
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    fn bytes_per_pixel() -> usize {
        3
    }

    pub fn new(filename: &str) -> ImageTexture {
        let img = ImageReader::open(filename).unwrap().decode().unwrap();
        ImageTexture {
            data: img.to_bytes(),
            width: img.dimensions().0 as usize,
            height: img.dimensions().1 as usize,
            bytes_per_scanline: Self::bytes_per_pixel() * (img.dimensions().0 as usize),
        }
    }
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        self.color_value
    }
}

impl Texture for CheckerTexture {
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

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * point.z + 10.0 * self.noise.turb(&point, 7)).sin())
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Vec3) -> Color {
        let uu = clamp(u, 0.0, 1.0);
        let vv = 1.0 - clamp(v, 0.0, 1.0);
        let col_index = min((uu * self.width as f64) as usize, self.width - 1);
        let row_index = min((vv * self.height as f64) as usize, self.height - 1);
        let pixel_index =
            (self.bytes_per_scanline * row_index) + (col_index * Self::bytes_per_pixel());
        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * self.data[pixel_index] as f64,
            color_scale * self.data[pixel_index + 1] as f64,
            color_scale * self.data[pixel_index + 2] as f64,
        )
    }
}
pub struct Textures {
    pub v_solid_colors: Vec<SolidColor>,
    pub v_checker_textures: Vec<CheckerTexture>,
    pub v_noise_textures: Vec<NoiseTexture>,
    pub v_image_textures: Vec<ImageTexture>,
}

impl<'a> Textures {
    pub fn new() -> Textures {
        Textures {
            v_solid_colors: vec![
                SolidColor {
                    color_value: Vec3::new(0.48, 0.83, 0.53),
                },
                SolidColor {
                    color_value: Vec3::new(7.0, 7.0, 7.0),
                },
                SolidColor {
                    color_value: Vec3::new(0.7, 0.3, 0.1),
                },
                SolidColor {
                    color_value: Vec3::new(0.2, 0.4, 0.9),
                },
                SolidColor {
                    color_value: Vec3::new(1.0, 1.0, 1.0),
                },
                SolidColor {
                    color_value: Vec3::new(0.73, 0.73, 0.73),
                },
            ],
            v_checker_textures: Default::default(),
            v_noise_textures: vec![NoiseTexture::new(0.1)],
            v_image_textures: vec![ImageTexture::new("earthmap.jpeg")],
        }
    }
}
