use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> image::Rgb<u8> {
        let intensity: Interval = Interval::new(0.000, 0.999);
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;
        if r.is_nan() {
            r = 0.0;
        }
        if g.is_nan() {
            g = 0.0;
        }
        if b.is_nan() {
            b = 0.0;
        }
        let r = (intensity.clamp(linear_to_gamma(r)) * 256.0) as u8;
        let g = (intensity.clamp(linear_to_gamma(g)) * 256.0) as u8;
        let b = (intensity.clamp(linear_to_gamma(b)) * 256.0) as u8;
        image::Rgb([r, g, b])
    }
    pub fn white() -> Color {
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
    pub fn black() -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        f64::sqrt(linear)
    } else {
        0.0
    }
}
