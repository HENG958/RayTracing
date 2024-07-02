use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{reflect, refract, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: a.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction: Vec3 = rec.normal.clone() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        let scattered: Ray = Ray::new(rec.p.clone(), scatter_direction);
        let attenuation: Color = self.albedo.clone();
        Some((scattered, attenuation))
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        let fuzz: f64 = if f < 1.0 { f } else { 1.0 };
        Self {
            albedo: a.clone(),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(&r_in.direction().unit(), &rec.normal);
        let scattered: Ray = Ray::new(
            rec.p.clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        let attenuation: Color = self.albedo.clone();
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = r_in.direction().unit();
        let refracted: Vec3 = refract(&unit_direction, &rec.normal, refraction_ratio);

        let scattered: Ray = Ray::new(rec.p.clone(), refracted);
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}
