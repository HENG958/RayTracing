use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::interval::Interval;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn _clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = ray_t.max;
        for object in &self.objects {
            if let Some(tmp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = tmp_rec.t;
                rec = Some(tmp_rec);
            }
        }
        rec
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
