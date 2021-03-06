use super::interpolation::*;
use super::random::*;
use super::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: std::vec::Vec<Vec3>,
    perm_x: std::vec::Vec<i32>,
    perm_y: std::vec::Vec<i32>,
    perm_z: std::vec::Vec<i32>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            ranfloat: random_v_vec3(POINT_COUNT),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let depth = 2;
        let i = p.x().floor() as usize & 255;
        let j = p.y().floor() as usize & 255;
        let k = p.z().floor() as usize & 255;
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let mut c = [Default::default(); 8];

        for di in 0..depth {
            for dj in 0..depth {
                for dk in 0..depth {
                    let index = (self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]) as usize;
                    c[di * depth + dj * depth + dk] = self.ranfloat[index];
                }
            }
        }
        perlin_interp(&c, depth, depth, depth, u, v, w)
    }

pub fn turb(&self, p: Vec3, depth: usize) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = p.clone();
    let mut weight = 1.0;
    for _ in 0..depth {
        accum += weight * self.noise(temp_p);
        weight *= 0.5;
        temp_p *= 2.0;
    }
    libm::fabs(accum)
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
        for i in n - 1..0 {
            let target = random_int_in_limit(0, i as isize) as usize;
            p[i] ^= p[target];
            p[i] ^= p[target];
            p[i] ^= p[target];
        }
    }
}
