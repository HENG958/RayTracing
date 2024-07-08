use crate::aabb::AABB;
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }
    pub fn _new_tex(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_tex(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec1;
        let mut rec2;
        let rec = self.boundary.hit(r, Interval::universe());
        match rec {
            None => {
                return None;
            }
            Some(record) => {
                rec1 = record;
            }
        }
        let rec = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY));
        match rec {
            None => {
                return None;
            }
            Some(record) => {
                rec2 = record;
            }
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_len = r.direction().length();
        let dis_in_boundary = (rec2.t - rec1.t) * ray_len;
        let hit_dis =
            self.neg_inv_density * f64::log(thread_rng().gen_range(0.0..1.0), std::f64::consts::E);

        if hit_dis > dis_in_boundary {
            return None;
        }
        let t = rec1.t + hit_dis / ray_len;

        let rec: HitRecord = HitRecord {
            p: r.at(t),
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat: self.phase_function.clone(),
            t,
            front_face: true,
            u: rec1.u,
            v: rec1.v,
        };
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
