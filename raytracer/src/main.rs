use camera::CameraConfig;
use camera::ImageConfig;
use console::style;
use std::{fs::File, process::exit};
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod medium;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_stb_image;
pub mod sphere;
pub mod texture;
pub mod translate;
pub mod vec3;
use bvh::BvhNode;
use camera::Camera;
use color::Color;
use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use medium::ConstantMedium;
use quad::{cobox, Quad};
use rand::{thread_rng, Rng};
use sphere::Sphere;
use std::sync::Arc;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use translate::{RotateY, Translate};
use vec3::Point3;
use vec3::Vec3;

fn bouncing_sphere() {
    let path = std::path::Path::new("output/book2/imagetest.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(&color::Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(&color::Color::new(0.9, 0.9, 0.9))),
    ));

    let material_ground = Arc::new(Lambertian::new_texture(checker));
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(&color::Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(&color::Color::new(0.7, 0.6, 0.5), 0.0));

    let mut world = hittable_list::HittableList::new();
    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    world.add(Arc::new(sphere::Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(&albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                if choose_mat < 0.8 {
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::_new_moving(
                        &center,
                        0.2,
                        sphere_material,
                        &center2,
                    )));
                } else {
                    world.add(Arc::new(sphere::Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }

    let world = hittable_list::HittableList::new_form(Arc::new(BvhNode::from_list(&mut world)));
    let image_setting = camera::ImageConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_setting = camera::CameraConfig {
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 10.0,
    };

    let mut camera = Camera::new(image_setting, camera_setting);
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
fn earth() {
    if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        bouncing_sphere();
    }
    let path = std::path::Path::new("output/book2/image5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));

    let mut world = hittable_list::HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    let world = hittable_list::HittableList::new_form(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraConfig {
        vfov: 20.0,
        look_from: Point3::new(0.0, 0.0, 12.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world);

    println!(
        "Output image as \"{}\"",
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
fn perlin() {
    let path = std::path::Path::new("output/book2/image15.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let mut world = hittable_list::HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_texture(pertext)),
    )));
    let world = hittable_list::HittableList::new_form(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraConfig {
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world);

    println!(
        "Output image as \"{}\"",
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

fn quad() {
    if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        bouncing_sphere();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        earth();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        perlin();
    }
    let path = std::path::Path::new("output/book2/image16.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let left_red = Arc::new(Lambertian::new(&Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_texture(pertext));
    let right_blue = Arc::new(Lambertian::new(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(&Color::new(0.2, 0.8, 0.8)));

    let mut world = hittable_list::HittableList::new();
    world.add(Arc::new(Quad::new(
        &Point3::new(-3.0, -2.0, 5.0),
        &Vec3::new(0.0, 0.0, -4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -2.0, 0.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(3.0, -2.0, 1.0),
        &Vec3::new(0.0, 0.0, 4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, 3.0, 1.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -3.0, 5.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    let world = hittable_list::HittableList::new_form(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageConfig {
        aspect_ratio: 1.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraConfig {
        vfov: 80.0,
        look_from: Point3::new(0.0, 0.0, 9.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world);

    println!(
        "Output image as \"{}\"",
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

fn main() {
    if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        bouncing_sphere();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        earth();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        perlin();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        quad();
    }
    let path = std::path::Path::new("output/book2/image22.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let diffuse = Arc::new(DiffuseLight::new(&Color::new(7.0, 7.0, 7.0)));
    let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let mut world = hittable_list::HittableList::new();
    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(3.0, 1.0, -2.0),
        &Vec3::new(2.0, 0.0, 0.0),
        &Vec3::new(0.0, 2.0, 0.0),
        diffuse.clone(),
    )));

    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(2.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        &Point3::new(113.0, 554.0, 127.0),
        &Vec3::new(330.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 305.0),
        diffuse,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = cobox(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        &Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = cobox(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );

    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let world = hittable_list::HittableList::new_form(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        quality: 100,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::new(0.0, 0.0, 0.0),
    };

    let camera_settings = CameraConfig {
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -900.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_distance: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world);

    println!(
        "Output image as \"{}\"",
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
