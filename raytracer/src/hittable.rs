use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Vec3, t: f64, outward_normal: &Vec3, r: &Ray) -> Self {
        let front_face: bool = r.direction().dot(outward_normal) < 0.0;
        let mut normal: Vec3 = outward_normal.clone();
        if !front_face {
            normal = -normal;
        }
        Self {
            p: p.clone(),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
