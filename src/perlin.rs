use super::random::*;
use super::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::random_in_limit(-1.0, 1.0).unit_vector());
        }
        Perlin {
            ranvec: ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = u as i64;
        let j = v as i64;
        let k = w as i64;
        let mut c = Vec::with_capacity(8);
        let mask = 255;
        let max = 2;
        for di in 0..max {
            for dj in 0..max {
                for dk in 0..max {
                    c.push(
                        self.ranvec[self.perm_x[((i + di) & mask) as usize]
                            ^ self.perm_y[((j + dj) & mask) as usize]
                            ^ self.perm_z[((k + dk) & mask) as usize]],
                    );
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }
        accum.abs()
    }

    #[allow(dead_code)]
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

    fn perlin_interp(c: &[Vec3], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        let max = 2;
        for i in 0..max {
            let f_i = i as f64;
            for j in 0..max {
                let f_j = j as f64;
                for k in 0..max {
                    let f_k = k as f64;
                    let weight_v = Vec3::new(u - f_i, v - f_j, w - f_k);
                    let f_k = k as f64;
                    let index = (i * max) + (j * max) + k;
                    accum += (f_i * uu + (1.0 - f_i) * (1.0 - uu))
                        * (f_j * vv + (1.0 - f_j) * (1.0 - vv))
                        * (f_k * ww + (1.0 - f_k) * (1.0 - ww))
                        * c[index].dot(&weight_v);
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
