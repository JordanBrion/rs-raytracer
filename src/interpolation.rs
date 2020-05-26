use super::vec3::*;

pub fn trilinear_interp(c: &[f64], x: usize, y: usize, z: usize, u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..x {
        let fi = i as f64;
        for j in 0..y {
            let fj = j as f64;
            for k in 0..z {
                let fk = k as f64;
                accum += (fi * u + (1.0 - fi) * (1.0 - u))
                    * (fj * v + (1.0 - fj) * (1.0 - v))
                    * (fk * w + (1.0 - fk) * (1.0 - w))
                    * c[i * x + j * y + k]
            }
        }
    }
    accum
}

pub fn perlin_interp(c: &[Vec3], x: usize, y: usize, z: usize, u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..x {
        let fi = i as f64;
        for j in 0..y {
            let fj = j as f64;
            for k in 0..z {
                let fk = k as f64;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * c[i * x + j * y + k].dot(&weight_v)
            }
        }
    }
    accum
}
