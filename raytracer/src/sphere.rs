use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center.clone() - r.origin().clone();
        let a: f64 = r.direction().length_squared();
        let h: f64 = r.direction().dot(&oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (h - sqrtd) / a;
        if root < ray_t.min || ray_t.max < root {
            root = (h + sqrtd) / a;
            if root < ray_t.min || ray_t.max < root {
                return None;
            }
        }

        let t: f64 = root;
        let p: Point3 = r.at(t);
        let outward_normal: Vec3 = (p.clone() - self.center.clone()) / self.radius;

        let rec: HitRecord = HitRecord::new(&p, t, &outward_normal, r);
        Some(rec)
    }
}
