use rand::{thread_rng, Rng};
use std::f64;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vec3 {
    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn _x(&self) -> f64 {
        self.x
    }
    pub fn _y(&self) -> f64 {
        self.y
    }
    pub fn _z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }
    pub fn random_in(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn _near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}
pub fn _random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
pub fn random_cosine_direction() -> Vec3 {
    let r1: f64 = thread_rng().gen_range(0.0..1.0);
    let r2: f64 = thread_rng().gen_range(0.0..1.0);

    let phi = 2.0 * r1 * std::f64::consts::PI;
    Vec3 {
        x: phi.cos() * r2.sqrt(),
        y: phi.sin() * r2.sqrt(),
        z: (1.0 - r2).sqrt(),
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v, n)
}
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&(-*uv), n), 1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())));
    r_out_perp + r_out_parallel
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

pub type Point3 = Vec3;