mod camera;
mod hittable;
mod material;
mod ppm;
mod random;
mod ray;
mod sphere;
mod vec3;
mod world;
mod constants;
mod angles;

use camera::*;
use hittable::*;
use material::*;
use ppm::*;
use random::*;
use ray::*;
use vec3::*;
use world::*;
use constants::*;

fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.0, INFINITY) {
        let target = record.p + record.normal + random_in_unit_sphere();
        let new_ray = Ray::new(record.p, target - record.p);
        return 0.5 * ray_color(&new_ray, world);
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + random_double()) / ppm.width as f32;
                let v = (j as f32 + random_double()) / ppm.height as f32;
                let ray = camera.get_ray(u, v);
                c += ray_color(&ray, &world);
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
