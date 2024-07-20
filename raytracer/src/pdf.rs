use crate::onb::Onb;
use crate::vec3::{random_cosine_direction, Vec3, Point3};
use crate::hittable::Hittable;
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, _dir: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct SpherePDF {}

impl SpherePDF {
    fn _new() -> Self {
        Self {}
    }
}

impl Pdf for SpherePDF {
    fn value(&self, _dir: &Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: Onb,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        let uvw = Onb::new(w);
        Self { uvw }
    }
}

impl Pdf for CosinePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        let cosine_theta = dir.unit().dot(&self.uvw.w());
        f64::max(0.0, cosine_theta / std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable>, origin: &Point3) -> Self {
        Self {
            objects,
            origin: *origin,
        }
    }
}

impl Pdf for HittablePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, dir)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}