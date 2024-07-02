use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{cross, random_in_unit_disk, Point3, Vec3};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

pub struct ImageConfig {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub quality: u8,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
}

pub struct CameraConfig {
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

pub struct Camera {
    // image
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub quality: u8,
    pub img: RgbImage,
    // Camera & Viewport
    pub camera_center: Point3,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub theta: f64,
    pub h: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Point3,
    pub pixel100_loc: Point3,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub pixel_samples_scale: f64,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(image_setting: ImageConfig, camera_setting: CameraConfig) -> Self {
        let ImageConfig {
            aspect_ratio,
            image_width,
            quality,
            samples_per_pixel,
            max_depth,
        } = image_setting;
        let CameraConfig {
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance,
        } = camera_setting;
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height == 0 {
            image_height = 1;
        }

        let camera_center: Point3 = look_from.clone();
        let theta: f64 = vfov * std::f64::consts::PI / 180.0;
        let h: f64 = f64::tan(theta / 2.0);

        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;
        let viewport_height: f64 = 2.0 * h * focus_distance;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        // edge vector
        let w = (look_from.clone() - look_at.clone()).unit();
        let u = cross(&vup, &w).unit();
        let v = cross(&w, &u);
        // viewport
        let viewport_u: Vec3 = u.clone() * viewport_width;
        let viewport_v: Vec3 = v.clone() * -viewport_height;
        // delta vector
        let pixel_delta_u: Vec3 = viewport_u.clone() / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v.clone() / image_height as f64;
        // upper left
        let viewport_upper_left: Point3 = camera_center.clone()
            - w * focus_distance
            - viewport_u.clone() / 2.0
            - viewport_v.clone() / 2.0;
        let pixel100_loc: Point3 =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;
        let defocus_radius =
            focus_distance * f64::tan(defocus_angle / 2.0 * std::f64::consts::PI / 180.0);
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            quality,
            img: ImageBuffer::new(image_width, image_height),
            viewport_height,
            viewport_width,
            camera_center,
            look_from,
            look_at,
            vup,
            vfov,
            theta,
            h,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel100_loc,
            max_depth,
            samples_per_pixel,
            pixel_samples_scale,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
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
                progress.inc(1);
            }
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin.clone();
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.camera_center.clone()
            + (self.defocus_disk_u.clone() * p.x)
            + (self.defocus_disk_v.clone() * p.y)
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
