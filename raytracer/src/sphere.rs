use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: AABB,
}

impl Sphere {
    pub(crate) fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox = AABB::two_point(&(center.clone() - r_vec.clone()), &(center.clone() + r_vec));
        Self {
            center: center.clone(),
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
            bbox,
        }
    }

    pub(crate) fn _new_moving(
        center: &Point3,
        radius: f64,
        mat: Arc<dyn Material>,
        center2: &Vec3,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::two_point(
            &(center.clone() - r_vec.clone()),
            &(center.clone() + r_vec.clone()),
        );
        let bbox2 = AABB::two_point(
            &(center2.clone() - r_vec.clone()),
            &(center2.clone() + r_vec),
        );
        let bbox = AABB::two_aabb(&bbox1, &bbox2);
        Self {
            center: center.clone(),
            radius,
            mat,
            is_moving: true,
            center_vec: center2.clone() - center.clone(),
            bbox,
        }
    }

    pub(crate) fn sphere_center(&self, time: f64) -> Point3 {
        if self.is_moving {
            self.center.clone() + self.center_vec.clone() * (time)
        } else {
            self.center.clone()
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center: Vec3 = self.sphere_center(r.time());
        let oc: Vec3 = center - r.origin().clone();
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

        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        let rec: HitRecord = HitRecord::new(&p, t, &outward_normal, r, self.mat.clone(), u, v);
        Some(rec)
    }
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
