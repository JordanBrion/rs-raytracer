mod ppm;
mod vec3;

use ppm::*;
use vec3::*;

fn main() {
    let mut ppm = PPM::new(100, 200);



    for j in 0..ppm.height {
        for i in 0..ppm.width {
            let v = Vec3::new(i as f32 / ppm.width as f32, j as f32 / ppm.height as f32, 0.2);
            let ir = (255.99 * v.r()) as u8;
            let ig = (255.99 * v.g()) as u8;
            let ib = (255.99 * v.b()) as u8;

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
