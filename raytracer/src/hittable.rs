use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Vec3, t: f64, outward_normal: &Vec3, r: &Ray, mat: Arc<dyn Material>) -> Self {
        let front_face: bool = r.direction().dot(outward_normal) < 0.0;
        let mut normal: Vec3 = outward_normal.clone();
        if !front_face {
            normal = -normal;
        }
        Self {
            p: p.clone(),
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, rat_t: Interval) -> Option<HitRecord>;
}
