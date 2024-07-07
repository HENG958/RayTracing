use crate::vec3::{Point3, Vec3};
use rand::{thread_rng, Rng};

pub struct Perlin {
    rand_vec: Vec<Vec3>,
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
        let mut rand_vec: Vec<Vec3> = vec![];
        for _i in 0..Self::POINT_COUNT {
            rand_vec.push(Vec3::random_in(-1.0, 1.0));
        }

        Self {
            rand_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(1.0, 1.0, 1.0); 2]; 2]; 2];

        for (di, c1) in c.iter_mut().enumerate() {
            for (dj, c2) in c1.iter_mut().enumerate() {
                for (dk, c3) in c2.iter_mut().enumerate().take(2usize) {
                    *c3 = self.rand_vec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
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

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * cijk.dot(&weight_v);
                }
            }
        }
        accum
    }
}
