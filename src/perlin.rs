use super::random::*;
use super::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(random_double());
        }
        Perlin {
            ranfloat: ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let i = uu as i64;
        let j = vv as i64;
        let k = ww as i64;
        let mut c = Vec::with_capacity(8);
        let mask = 255;
        let max = 2;
        for di in 0..max {
            for dj in 0..max {
                for dk in 0..max {
                    c.push(
                        self.ranfloat[self.perm_x[((i + di) & mask) as usize]
                            ^ self.perm_y[((j + dj) & mask) as usize]
                            ^ self.perm_z[((k + dk) & mask) as usize]],
                    );
                }
            }
        }
        Self::trilinear_interp(&c, uu, vv, ww)
    }

    fn trilinear_interp(c: &[f64], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let max = 2;
        for i in 0..max {
            let f_i = i as f64;
            for j in 0..max {
                let f_j = j as f64;
                for k in 0..max {
                    let f_k = k as f64;
                    let index = (i * max) + (j * max) + k;
                    accum += (f_i * u + (1.0 - f_i) * (1.0 - u))
                        * (f_j * v + (1.0 - f_j) * (1.0 - v))
                        * (f_k * w + (1.0 - f_k) * (1.0 - w))
                        * c[index];
                }
            }
        }
        accum
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (0..n).rev() {
            let target = random_integer_in_limit(0, i);
            p.swap(i, target);
        }
    }
}
