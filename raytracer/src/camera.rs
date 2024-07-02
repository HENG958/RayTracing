use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

pub struct Camera {
    // image
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub quality: u8,
    pub img: RgbImage,
    // Camera & Viewport
    pub focal_length: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub camera_center: Point3,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Point3,
    pub pixel100_loc: Point3,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        quality: u8,
        samples_per_pixel: u32,
        max_depth: i32,
        focal_length: f64,
        viewport_height: f64,
    ) -> Self {
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height == 0 {
            image_height = 1;
        }
        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
        // edge vector
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
        // delta vector
        let pixel_delta_u: Vec3 = viewport_u.clone() / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v.clone() / image_height as f64;
        // upper left
        let viewport_upper_left: Point3 = camera_center.clone()
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u.clone() / 2.0
            - viewport_v.clone() / 2.0;
        let pixel100_loc: Point3 =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            quality,
            img: ImageBuffer::new(image_width, image_height),
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel100_loc,
            max_depth,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&mut self, world: HittableList) -> &RgbImage {
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + ray_color(r, self.max_depth, &world);
                }
                pixel_color *= self.pixel_samples_scale;
                let pixel = self.img.get_pixel_mut(i, j);
                *pixel = pixel_color.write_color();
            }
            progress.inc(1);
        }
        progress.finish();
        &self.img
    }

    pub fn get_ray(&self, u: u32, v: u32) -> Ray {
        let offset: Vec3 = sample_square();
        let u: f64 = (u as f64) + offset.x;
        let v: f64 = (v as f64) + offset.y;
        let pixel_center: Point3 = self.pixel100_loc.clone()
            + (self.pixel_delta_u.clone() * u)
            + (self.pixel_delta_v.clone() * v);
        let ray_direction: Vec3 = pixel_center - self.camera_center.clone();
        Ray::new(self.camera_center.clone(), ray_direction)
    }
}

fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth < 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(&r, &rec) {
            return attenuation * ray_color(scattered, depth - 1, world);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
}
