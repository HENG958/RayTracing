use crate::color::Color;
use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::rtw_stb_image::RTWImage;
use crate::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
    pub fn _new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(&Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn _new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_color(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x: i32 = (self.inv_scale * p.x).floor() as i32;
        let y: i32 = (self.inv_scale * p.y).floor() as i32;
        let z: i32 = (self.inv_scale * p.z).floor() as i32;

        let is_even: bool = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    pub fn new(file_name: &str) -> Self {
        Self {
            image: RTWImage::new(file_name),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        };

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;

        Color::new(
            gamma_to_linear(color_scale * pixel[0] as f64),
            gamma_to_linear(color_scale * pixel[1] as f64),
            gamma_to_linear(color_scale * pixel[2] as f64),
        )
    }
}

fn gamma_to_linear(linear: f64) -> f64 {
    if linear > 0.0 {
        linear * linear
    } else {
        0.0
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(_scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: _scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}