mod camera;
mod hittable;
mod material;
mod ppm;
mod random;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::*;
use hittable::*;
use material::*;
use ppm::*;
use random::*;
use ray::*;
use vec3::*;
use world::*;

fn color(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.001, std::f32::MAX) {
        let mut scattered = Default::default();
        let mut attenuation = Default::default();
        if depth < 50
            && record
                .material
                .scatter(ray, &record, &mut attenuation, &mut scattered)
        {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut ppm = PPM::new(100, 200);
    let r = (std::f64::consts::PI as f32 / 4.0).cos();
    let camera = Camera::new(90.0, ppm.width as f32 / ppm.height as f32);
    let materials = Materials::new();
    let world = World::new(&materials, r);
    let samples = 100;

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + random_number()) / ppm.width as f32;
                let v = (j as f32 + random_number()) / ppm.height as f32;
                let ray = camera.get_ray(u, v);
                c += color(&ray, &world, 0);
            }
            c /= samples as f32;
            c = Vec3::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            let final_y = ppm.height - j;
            ppm.set_pixel(
                i,
                final_y,
                RGB {
                    r: ir,
                    g: ig,
                    b: ib,
                },
            );
        }
    }
    ppm.write_file("test.ppm").expect("Cannot write ppm file!!");
}
