use crate::vec3::Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> image::Rgb<u8> {
        let r: u8 = (256.0 * self.x.sqrt().clamp(0.0, 0.999)) as u8;
        let g: u8 = (256.0 * self.y.sqrt().clamp(0.0, 0.999)) as u8;
        let b: u8 = (256.0 * self.z.sqrt().clamp(0.0, 0.999)) as u8;
        image::Rgb([r, g, b])
    }
}
