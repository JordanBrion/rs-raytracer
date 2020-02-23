extern crate rand;

mod camera;
mod hittable;
mod ppm;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::*;
use hittable::*;
use ppm::*;
use rand::Rng;
use ray::*;
use sphere::*;
use vec3::*;
use world::*;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(random_number(), random_number(), random_number()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

fn color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.001, std::f32::MAX) {
        let target = record.p + record.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(record.p, target - record.p), world)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn random_number() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

fn main() {
    let mut ppm = PPM::new(100, 200);
    let camera = Camera::new();
    let world = World::new();
    let samples = 100;

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + random_number()) / ppm.width as f32;
                let v = (j as f32 + random_number()) / ppm.height as f32;
                let ray = camera.get_ray(u, v);
                c += color(&ray, &world);
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
