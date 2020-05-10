extern crate num;

mod angles;
mod camera;
mod constants;
mod hittable;
mod material;
mod ppm;
mod random;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::*;
use constants::*;
use hittable::*;
use material::*;
use ppm::*;
use random::*;
use ray::*;
use vec3::*;
use world::*;

fn color(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    } else if let Some(record) = world.hit(ray, 0.0, INFINITY) {
        let target = record.p + record.normal + random_in_unit_sphere();
        let new_ray = Ray::new(record.p, target - record.p);
        return 0.5 * ray_color(&new_ray, world, depth);
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
    }
}

fn gamma_correction(color: Vec3, samples_per_pixel: i32) -> RGB {
    let scale = 1.0 / samples_per_pixel as f32;
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
    let r = (PI / 4.0).cos();
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ppm.width as f32 / ppm.height as f32,
        2.0,
        (look_from - look_at).length(),
    );
    let materials = Materials::new();
    let world = World::new(&materials, r);
    let samples = 100;
    let max_depth = 50;

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + random_double()) / ppm.width as f32;
                let v = (j as f32 + random_double()) / ppm.height as f32;
                let ray = camera.get_ray(u, v);
                c += ray_color(&ray, &world, max_depth);
            }
            ppm.set_pixel(i, ppm.height - j, gamma_correction(c, samples));
        }
    }
    ppm.write_file("test.ppm").expect("Cannot write ppm file!!");
}
