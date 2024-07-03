pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, tm: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + self.dir.clone() * t
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}
