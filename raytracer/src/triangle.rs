use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct Triangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Triangle {
    pub fn new(q: &Point3, a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Self {
        let bbox1 = Aabb::two_point(q, a);
        let bbox2 = Aabb::two_point(q, b);

        let u = *a - *q;
        let v = *b - *q;
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let d = dot(&normal, q);
        let w = n / dot(&n, &n);
        let area = n.length() / 2.0;
        Self {
            q: *q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::two_aabb(&bbox1, &bbox2),
            normal,
            d,
            area,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, &r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let p_q = intersection - self.q;
        let alpha = dot(&self.w, &cross(&p_q, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &p_q));

        let range = Interval::new(0.0, 1.0);
        if !range.contains(alpha) || !range.contains(beta) || !range.contains(alpha + beta) {
            return None;
        }
        let rec = HitRecord::new(
            &intersection,
            t,
            &self.normal,
            r,
            self.mat.clone(),
            alpha,
            beta,
        );
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if let Some(rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            Interval::new(0.001, f64::INFINITY),
        ) {
            let distance_squared = rec.t * rec.t * direction.length_squared();
            let cosine = dot(direction, &rec.normal).abs() / direction.length();

            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let a = thread_rng().gen_range(0.0..1.0);
        let b = thread_rng().gen_range(0.0..1.0 - a);
        let p = self.q + self.u * a + self.v * b;
        p - *origin
    }
}
