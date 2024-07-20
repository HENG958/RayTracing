use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::{CosinePDF, Pdf, SpherePDF};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Point3};
use rand::Rng;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::black()
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(&albedo)),
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf_ptr = Arc::new(CosinePDF::new(&rec.normal));
        let skip_pdf = false;
        let scatter_record = ScatterRecord {
            attenuation,
            pdf_ptr: Some(pdf_ptr),
            skip_pdf,
            skip_pdf_ray: None,
        };
        Some(scatter_record)
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &unit_vector(&scattered.direction()));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + random_unit_vector() * self.fuzz;
        let scattered = Ray::new(&rec.p, &reflected, r_in.time());
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            let srec = ScatterRecord {
                attenuation,
                pdf_ptr: None,
                skip_pdf: true,
                skip_pdf_ray: Some(scattered),
            };
            Some(srec)
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub(crate) fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    pub fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::white();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = f64::min(dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let mut rng = rand::thread_rng();
        let direction =
            if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        let scattered = Ray::new(&rec.p, &direction, r_in.time());

        let srec = ScatterRecord {
            attenuation,
            pdf_ptr: None,
            skip_pdf: true,
            skip_pdf_ray: Some(scattered),
        };
        Some(srec)
    }
}

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
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            Color::black()
        } else {
            self.tex.value(u, v, p)
        }
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn _new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf_ptr = Arc::new(SpherePDF::_new());
        let skip_pdf = false;
        let scatter_record = ScatterRecord {
            attenuation,
            pdf_ptr: Some(pdf_ptr),
            skip_pdf,
            skip_pdf_ray: None,
        };
        Some(scatter_record)
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}
