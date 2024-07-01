use console::style;
use std::{fs::File, process::exit};
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;
use camera::Camera;
use ray::Point3;
use std::rc::Rc;

fn main() {
    let path = std::path::Path::new("output/book1/image6.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(sphere::Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 10, 1.0, 2.0);
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
