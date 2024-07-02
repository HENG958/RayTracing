use console::style;
use std::{fs::File, process::exit};
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian};
use std::sync::Arc;
use vec3::Point3;

fn main() {
    let path = std::path::Path::new("output/book1/image19.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //let material_ground = Arc::new(material::Lambertian::new(&color::Color::new(0.8, 0.8, 0.0)));
    //let material_center = Arc::new(material::Lambertian::new(&color::Color::new(0.1, 0.2, 0.5)));
    //let material_left = Arc::new(material::Metal::new(&color::Color::new(0.8, 0.8, 0.8), 0.3));

    let _material_left = Arc::new(Dielectric::new(1.50));
    //let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let _material_right = Arc::new(material::Metal::new(&color::Color::new(0.8, 0.6, 0.2), 1.0));

    let r = f64::cos(std::f64::consts::PI / 4.0);

    let material_left = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(&Color::new(1.0, 0.0, 0.0)));

    let mut world = hittable_list::HittableList::new();
    // world.add(Arc::new(sphere::Sphere::new(
    //     &Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));

    // world.add(Arc::new(sphere::Sphere::new(
    //     &Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));

    // world.add(Arc::new(sphere::Sphere::new(
    //     &Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));

    // world.add(Arc::new(sphere::Sphere::new(
    //     &Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     material_bubble,
    // )));

    // world.add(Arc::new(sphere::Sphere::new(
    //     &Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));

    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 100, 50, 1.0, 90.0);
    camera.render(world);

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(camera.img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(camera.quality),
    ) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
