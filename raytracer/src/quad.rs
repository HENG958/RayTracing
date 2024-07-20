use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct Quad {
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

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox1 = Aabb::two_point(q, &(*q + *u + *v));
        let bbox2 = Aabb::two_point(&(*q + *u), &(*q + *v));

        let n = cross(u, v);
        let normal = unit_vector(&n);
        let d = dot(&normal, q);
        let w = n / dot(&n, &n);
        let area = n.length();
        Self {
            q: *q,
            u: *u,
            v: *v,
            w,
            mat,
            bbox: Aabb::two_aabb(&bbox1, &bbox2),
            normal,
            d,
            area,
        }
    }
}

impl Hittable for Quad {
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
        if !range.contains(alpha) || !range.contains(beta) {
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
        let p = self.q
            + (self.u * thread_rng().gen_range(0.0..1.0))
            + (self.v * thread_rng().gen_range(0.0..1.0));
        p - *origin
    }
}

pub fn cuboid(a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let mut sides = HittableList::new();

    let min = Point3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
    let max = Point3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        &Point3::new(min.x, min.y, max.z),
        &dx,
        &dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        &Point3::new(max.x, min.y, max.z),
        &-dz,
        &dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        &Point3::new(max.x, min.y, min.z),
        &-dx,
        &dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        &Point3::new(min.x, min.y, min.z),
        &dz,
        &dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        &Point3::new(min.x, max.y, max.z),
        &dx,
        &-dz,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        &Point3::new(min.x, min.y, min.z),
        &dx,
        &dz,
        mat,
    )));

    Arc::new(BvhNode::from_list(&mut sides))
}