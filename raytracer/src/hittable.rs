use crate::aabb::AABB;
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
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        p: &Vec3,
        t: f64,
        outward_normal: &Vec3,
        r: &Ray,
        mat: Arc<dyn Material>,
        u: f64,
        v: f64,
    ) -> Self {
        let front_face: bool = r.direction().dot(outward_normal) < 0.0;
        let mut normal: Vec3 = *outward_normal;
        if !front_face {
            normal = -normal;
        }
        Self {
            p: *p,
            normal,
            mat,
            t,
            front_face,
            u,
            v,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, rat_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}