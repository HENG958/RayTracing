use image::{DynamicImage, GenericImageView};
use std::path::Path;

#[derive(Clone)]
pub struct RTWImage {
    image: Option<DynamicImage>,
    image_width: u32,
    image_height: u32,
}

impl RTWImage {
    pub fn new(image_filename: &str) -> Self {
        let mut rtw_image = RTWImage {
            image: None,
            image_width: 0,
            image_height: 0,
        };

        let filename = String::from(image_filename);

        if rtw_image.load(&filename) {
            return rtw_image;
        }
        if rtw_image.load(&format!("images/{}", filename)) {
            return rtw_image;
        }

        eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        rtw_image
    }

    fn load(&mut self, filename: &str) -> bool {
        match image::open(Path::new(filename)) {
            Ok(img) => {
                self.image_width = img.width();
                self.image_height = img.height();
                self.image = Some(img);
                true
            }
            Err(_) => false,
        }
    }

    pub fn width(&self) -> u32 {
        self.image_width
    }

    pub fn height(&self) -> u32 {
        self.image_height
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if let Some(ref img) = self.image {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            let pixel = img.get_pixel(x, y).0;
            [pixel[0], pixel[1], pixel[2]]
        } else {
            MAGENTA
        }
    }

    pub fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}
