mod ppm;
mod ray;
mod vec3;
mod sphere;
mod hittable;
mod world;

use ppm::*;
use ray::*;
use vec3::*;
use sphere::*;
use world::*;
use hittable::*;

fn color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.0, std::f32::MAX) {
        record.normal.make_color()
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut ppm = PPM::new(100, 200);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world = World::new();

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let u = i as f32 / ppm.width as f32;
            let v = j as f32 / ppm.height as f32;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let c = color(&ray, &world);
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
