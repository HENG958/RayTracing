use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: AABB::zero(),
        }
    }
    pub fn _clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::two_aabb(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn new_form(objects: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(objects);
        list
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

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut size: u32 = 0;
        let mut sum: f64 = 0.0;
        for object in self.objects.iter() {
            sum += object.pdf_value(origin, direction);
            size += 1;
        }
        sum /= size as f64;
        sum
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let size = self.objects.len();
        self.objects[thread_rng().gen_range(0..size)].random(origin)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}