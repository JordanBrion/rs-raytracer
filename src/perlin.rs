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
        let factor = 4.0;
        let mask = 0xFF_u64;
        let i = ((factor * p.x).to_bits() & mask) as usize;
        let j = ((factor * p.y).to_bits() & mask) as usize;
        let k = ((factor * p.z).to_bits() & mask) as usize;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
