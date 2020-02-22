mod ppm;

use ppm::*;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let v1 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let nx = 200;
    let ny = 100;
    let mut ppm = PPM::new(ny, nx);

    for j in 0..ny {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

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
