mod ppm;
mod vec3;

use ppm::*;
use vec3::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut ppm = PPM::new(ny, nx);

    for j in 0..ny {
        for i in 0..nx {
            let v = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let ir = (255.99 * v.r()) as u8;
            let ig = (255.99 * v.g()) as u8;
            let ib = (255.99 * v.b()) as u8;

            let final_y = ny - j;
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
