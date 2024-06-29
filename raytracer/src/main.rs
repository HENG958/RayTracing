use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};
pub mod color;
pub mod vec3;
use color::Color;
pub mod ray;
use ray::{Point3, Ray};
use vec3::Vec3;

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = center.clone() - r.origin().clone();
    let a = r.direction().length_squared();
    let b = -2.0 * oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
pub fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as u32;
    //if image_height < 1 {image_height = 1;}
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    let viewport_origin = camera_center.clone()
        - viewport_u / 2.0
        - viewport_v / 2.0
        - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_origin + pixel_delta_u.clone() / 2.0 + pixel_delta_v.clone() / 2.0;

    //println!("P3\n{} {}\n255", image_width, image_height);

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_center = pixel00_loc.clone()
                + pixel_delta_u.clone() * i as f64
                + pixel_delta_v.clone() * j as f64;
            let r = Ray::new(
                camera_center.clone(),
                pixel_center.clone() - camera_center.clone(),
            );
            //println!("{:?}", r.direction());
            let pixel_color = ray_color(&r);
            //println!("{:?}", pixel_color);
            *pixel = pixel_color.write_color();
            //let pixel = img.get_pixel_mut(i, j)
            //let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            //let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            //let b: f64 = 0.25 * 255.999;
            //*pixel = write_color(&Vec3::new(r, g, b));
            //*pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
