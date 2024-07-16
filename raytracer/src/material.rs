use crate::color::Color;
use crate::hittable::HitRecord;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{random_cosine_direction, reflect, refract, Point3, Vec3};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        None
    }

    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl std::fmt::Debug for Lambertian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambertian {{ texture: ... }}")
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(a)),
        }
    }

    pub fn new_texture(tex: Arc<dyn Texture>) -> Self {
        Self { texture: tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let uvw: Onb = Onb::new(&rec.normal);
        let scatter_direction: Vec3 = uvw.local(&random_cosine_direction());

        let scattered: Ray = Ray::new(rec.p, scatter_direction, r_in.time());
        let attenuation: Color = self.texture.value(rec.u, rec.v, &rec.p);
        let pdf = uvw.w().dot(scattered.direction()) / std::f64::consts::PI;
        Some((scattered, attenuation, pdf))
    }

    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * std::f64::consts::PI)
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
        Self { albedo: *a, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let reflected: Vec3 = reflect(&r_in.direction().unit(), &rec.normal);
        let scattered: Ray = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        let attenuation: Color = self.albedo;
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, attenuation, 1.0))
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

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0: f64 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = r_in.direction().unit();
        let cos_theta: f64 = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered: Ray = Ray::new(rec.p, direction, r_in.time());
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation, 1.0))
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
    pub fn _new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }

    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        None
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: &Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { albedo: tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let scattered: Ray = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation: Color = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = 1.0 / (4.0 * std::f64::consts::PI);
        Some((scattered, attenuation, pdf))
    }

    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}
