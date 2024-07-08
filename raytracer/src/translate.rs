use crate::aabb::{add, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: &Vec3) -> Self {
        let bbox = add(&object.bounding_box(), offset);
        Self {
            object,
            offset: *offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p = self.offset + rec.p;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle * std::f64::consts::PI / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();

        let inf = f64::INFINITY;
        let mut min = Point3::new(inf, inf, inf);
        let mut max = Point3::new(-inf, -inf, -inf);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let test = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], test[c]);
                        max[c] = f64::max(max[c], test[c]);
                    }
                }
            }
        }
        bbox = AABB::two_point(&min, &max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut ori = *r.origin();
        let mut dir = *r.direction();

        ori[0] = r.origin()[0] * self.cos_theta - r.origin()[2] * self.sin_theta;
        ori[2] = r.origin()[0] * self.sin_theta + r.origin()[2] * self.cos_theta;
        dir[0] = r.direction()[0] * self.cos_theta - r.direction()[2] * self.sin_theta;
        dir[2] = r.direction()[0] * self.sin_theta + r.direction()[2] * self.cos_theta;
        let rotated_r = Ray::new(ori, dir, r.time());

        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            let mut p = rec.p;
            p[0] = rec.p[0] * self.cos_theta + rec.p[2] * self.sin_theta;
            p[2] = rec.p[0] * -self.sin_theta + rec.p[2] * self.cos_theta;

            let mut normal = rec.normal;
            normal[0] = rec.normal[0] * self.cos_theta + rec.normal[2] * self.sin_theta;
            normal[2] = rec.normal[0] * -self.sin_theta + rec.normal[2] * self.cos_theta;

            rec.p = p;
            rec.normal = normal;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
