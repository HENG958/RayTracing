use crate::vec3::Point3;
use rand::{thread_rng, Rng};

pub struct Perlin {
    rand_float: Vec<f64>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rand_float: Vec<f64> = vec![];
        for _i in 0..Self::POINT_COUNT {
            rand_float.push(thread_rng().gen_range(0.0..1.0));
        }

        Self {
            rand_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.rand_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
    fn perlin_generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = vec![];
        for i in 0..Self::POINT_COUNT {
            p.push(i as u32);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }
    fn permute(p: &mut [u32], n: usize) {
        for i in (1..n - 1).rev() {
            let target = thread_rng().gen_range(0..i);
            p.swap(i, target);
        }
    }
}
