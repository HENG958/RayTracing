use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::rtw_stb_image::RTWImage;
use crate::{
    color::Color,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: &Color) -> Self {
        Self {
            color_value: c.clone(),
        }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color_value.clone()
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_width: f64,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_width: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn new_rgb(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self {
            inv_width: 1.0 / scale,
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x: i32 = (self.inv_width * p.x) as i32;
        let y: i32 = (self.inv_width * p.y) as i32;
        let z: i32 = (self.inv_width * p.z) as i32;

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
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Color {
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
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(&(self.scale * p))
    }
}
