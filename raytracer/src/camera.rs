use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::pdf::{CosinePDF, Pdf};
use crate::ray::Ray;
use crate::vec3::{cross, random_in_unit_disk, Point3, Vec3};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ImageConfig {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub quality: u8,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub background: Color,
}

pub struct CameraConfig {
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}
#[derive(Clone)]
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
    pub sqrt_spp: u32,
    pub recip_sqrt_spp: f64,
    pub background: Color,
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
            background,
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

        let camera_center: Point3 = look_from;
        let theta: f64 = vfov * std::f64::consts::PI / 180.0;
        let h: f64 = f64::tan(theta / 2.0);

        //let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;
        let sqrt_spp: u32 = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale: f64 = 1.0 / (sqrt_spp * sqrt_spp) as f64;
        let recip_sqrt_spp: f64 = 1.0 / sqrt_spp as f64;
        let viewport_height: f64 = 2.0 * h * focus_distance;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        // edge vector
        let w = (look_from - look_at).unit();
        let u = cross(&vup, &w).unit();
        let v = cross(&w, &u);
        // viewport
        let viewport_u: Vec3 = u * viewport_width;
        let viewport_v: Vec3 = v * -viewport_height;
        // delta vector
        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;
        // upper left
        let viewport_upper_left: Point3 =
            camera_center - w * focus_distance - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
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
            sqrt_spp,
            recip_sqrt_spp,
            background,
            samples_per_pixel,
            pixel_samples_scale,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: HittableList) {
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let lines: Vec<Option<Vec<Color>>> = vec![None; self.image_height as usize];
        let lines = Arc::new(Mutex::new(lines));
        // let img = Arc::new(Mutex::new(self.img.clone()));
        let progress = Arc::new(Mutex::new(progress));

        let mut rend_lines = vec![];

        let image_width = self.image_width;
        for j in (0..self.image_height).rev() {
            let lines_clone = Arc::clone(&lines);
            // let img = Arc::clone(&img);
            let progress = Arc::clone(&progress);
            let world = world.clone();
            let copy = self.clone();
            let rend_line = thread::spawn(move || {
                let mut line: Vec<Color> = Vec::with_capacity(image_width as usize);
                for i in 0..image_width {
                    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                    for s_j in 0..copy.sqrt_spp {
                        for s_i in 0..copy.sqrt_spp {
                            let r = copy.get_ray(i, j, s_i, s_j);
                            pixel_color = pixel_color
                                + ray_color(r, copy.max_depth, &world, &copy.background);
                        }
                    }
                    pixel_color *= copy.pixel_samples_scale;

                    // let mut img = img.lock().unwrap();
                    // let pixel = img.get_pixel_mut(i, j);
                    // *pixel = pixel_color.write_color();
                    // drop(img);
                    line.push(pixel_color);

                    let progress = progress.lock().unwrap();
                    progress.inc(1);
                }
                let mut lines = lines_clone.lock().unwrap();
                lines[j as usize] = Some(line);
            });
            rend_lines.push(rend_line);
        }

        for rend_line in rend_lines {
            rend_line.join().unwrap();
        }

        progress.lock().unwrap().finish();

        let lines = Arc::try_unwrap(lines).expect("!").into_inner().unwrap();
        // let img = Some(Arc::try_unwrap(img).unwrap().into_inner().unwrap());
        // self.img = img.as_ref().unwrap().clone();
        for (j, line_option) in lines.into_iter().enumerate() {
            if let Some(line) = line_option {
                for (i, color) in line.into_iter().enumerate() {
                    let pixel = self.img.get_pixel_mut(i as u32, j as u32);
                    *pixel = color.write_color();
                }
            }
        }
    }
    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = (s_i as f64 + thread_rng().gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;
        let py = (s_j as f64 + thread_rng().gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;

        Vec3::new(px, py, 0.0)
    }

    pub fn get_ray(&self, u: u32, v: u32, s_i: u32, s_j: u32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let u: f64 = (u as f64) + offset.x;
        let v: f64 = (v as f64) + offset.y;
        let pixel_center: Point3 =
            self.pixel100_loc + (self.pixel_delta_u * u) + (self.pixel_delta_v * v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;
        let ray_time = thread_rng().gen_range(0.0..1.0);
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

fn ray_color(r: Ray, depth: i32, world: &dyn Hittable, background: &Color) -> Color {
    if depth < 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
        let color_from_emission = rec.mat.emitted(&r, &rec, rec.u, rec.v, &rec.p);
        return if let Some((scattered, attenuation, _pdf)) = rec.mat.scatter(&r, &rec) {
            let surface_pdf = CosinePDF::new(&rec.normal);
            let pdf_value = surface_pdf.value(scattered.direction());
            let scattered = Ray::new(rec.p, surface_pdf.generate(), r.time());
            color_from_emission
                + attenuation
                    * rec.mat.scatter_pdf(&r, &rec, &scattered)
                    * ray_color(scattered, depth - 1, world, background)
                    / pdf_value
        } else {
            color_from_emission
        };
    }
    *background
}

fn _sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
}
