mod ppm;
mod ray;
mod vec3;

use ppm::*;
use ray::*;
use vec3::*;

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(ray) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(ray: &Ray) -> bool {
    let sphere_position = Vec3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let rs = ray.origin - sphere_position;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * rs.dot(&ray.direction);
    let c = rs.dot(&rs) - sphere_radius * sphere_radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn main() {
    let mut ppm = PPM::new(100, 200);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let u = i as f32 / ppm.width as f32;
            let v = j as f32 / ppm.height as f32;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let c = color(&ray);
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
