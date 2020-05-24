use super::random::*;
use super::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: std::vec::Vec<f32>,
    perm_x: std::vec::Vec<i32>,
    perm_y: std::vec::Vec<i32>,
    perm_z: std::vec::Vec<i32>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            ranfloat: random_v_double(POINT_COUNT),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i: usize = (4.0 * p.x()) as usize & 255;
        let j: usize = (4.0 * p.y()) as usize & 255;
        let k: usize = (4.0 * p.z()) as usize & 255;
        let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        self.ranfloat[index]
    }

    fn perlin_generate_perm() -> std::vec::Vec<i32> {
        let mut p = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Self::permute(p.as_mut_slice(), POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in n-1..0 {
            let target = random_int_in_limit(0, i as isize) as usize;
            p[i] ^= p[target];
            p[i] ^= p[target];
            p[i] ^= p[target];
        }
    }
}
