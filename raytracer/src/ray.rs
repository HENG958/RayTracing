use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::f64;

#[derive(Debug)]
pub struct Ray {
    _orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, tm: f64) -> Self {
        Self {
            _orig: *origin,
            dir: *direction,
            tm,
        }
    }

    pub fn origin(&self) -> Point3 {
        self._orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self._orig + self.dir * t
    }
}