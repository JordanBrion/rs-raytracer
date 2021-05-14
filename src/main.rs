extern crate num;

mod aabb;
mod angles;
mod bvh;
mod camera;
mod color;
mod constants;
mod hittable;
mod material;
mod normal;
mod perlin;
mod ppm;
mod random;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod uv;
mod vec3;
mod world;

use bvh::*;
use camera::*;
use color::*;
use constants::*;
use hittable::*;
use material::*;
use ppm::*;
use random::*;
use ray::*;
use texture::*;
use vec3::*;
use world::*;

fn ray_color(ray: &Ray, background: &Color, hittable: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        Default::default()
    } else if let Some(record) = hittable.hit(ray, 0.0001, INFINITY) {
        let mut scattered = Default::default();
        let mut attenuation = Default::default();
        let emitted = record.material.emitted(record.u, record.v, &record.p);
        if record
            .material
            .scatter(&ray, &record, &mut attenuation, &mut scattered)
        {
            emitted + attenuation * ray_color(&scattered, background, hittable, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
    }
}

fn gamma_correction(color: Vec3, samples_per_pixel: i32) -> RGB {
    let scale = 1.0 / samples_per_pixel as f64;
    let r_scaled = (scale * color.x).sqrt();
    let g_scaled = (scale * color.y).sqrt();
    let b_scaled = (scale * color.z).sqrt();
    RGB {
        r: (256.0 * num::clamp(r_scaled, 0.0, 0.999)) as u8,
        g: (256.0 * num::clamp(g_scaled, 0.0, 0.999)) as u8,
        b: (256.0 * num::clamp(b_scaled, 0.0, 0.999)) as u8,
    }
}

fn main() {
    let mut ppm = PPM::new(100, 200);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ppm.width as f64 / ppm.height as f64,
        0.0,
        dist_to_focus,
        0.0,
        1.0,
    );
    let textures = Textures::new();
    let materials = Materials::new(&textures);
    let world = World::new_two_perlin_spheres(&materials);
    // let bvh_root = BvhNode::new(&mut world.to_list_of_hittables(), 0.0, 1.0);
    let samples = 10;
    let max_depth = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f64 + random_double()) / ppm.width as f64;
                let v = (j as f64 + random_double()) / ppm.height as f64;
                let ray = camera.get_ray(u, v);
                c += ray_color(&ray, &background, &world, max_depth);
            }
            ppm.set_pixel(i, ppm.height - j, gamma_correction(c, samples));
        }
    }
    ppm.write_file("test.ppm").expect("Cannot write ppm file!!");
}
