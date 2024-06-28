use crate::vec3::Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> image::Rgb<u8> {
        let r = (self.x() * 255.99) as u8;
        let g = (self.y() * 255.99) as u8;
        let b = (self.z() * 255.99) as u8;
        image::Rgb([r, g, b])
    }
}
