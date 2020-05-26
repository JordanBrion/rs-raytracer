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
