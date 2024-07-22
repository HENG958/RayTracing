use image::RgbImage;

pub fn edge_detection(img: RgbImage) -> RgbImage {
    let mut gray = RgbImage::new(img.width(), img.height());
    for j in (0..gray.height()).rev() {
        for i in 0..gray.width() {
            let pixel = img.get_pixel(i, j);
            let pixel_gray = gray.get_pixel_mut(i, j);
            let g = 0.299 * pixel[0] as f64 + 0.587 * pixel[1] as f64 + 0.114 * pixel[2] as f64; // from OpenCV
            pixel_gray[0] = g as u8;
            pixel_gray[1] = g as u8;
            pixel_gray[2] = g as u8;
        }
    }

    let gx: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let gy: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    // sober

    let mut result = RgbImage::new(img.width() - 2, img.height() - 2);
    let max = 80;

    for j in 0..gray.height() - 2 {
        for i in 0..gray.width() - 2 {
            let mut fx = 0;
            let mut fy = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    let pixel = gray.get_pixel(i + dx, j + dy);
                    let g = pixel[0];
                    fx += gx[dy as usize][dx as usize] * g as i32;
                    fy += gy[dy as usize][dx as usize] * g as i32;
                    let f = fx.abs() + fy.abs();
                    let result_pixel = result.get_pixel_mut(i, j);
                    result_pixel[0] = if f > max { 0 } else { 255 };
                    result_pixel[1] = if f > max { 0 } else { 255 };
                    result_pixel[2] = if f > max { 0 } else { 255 };
                }
            }
        }
    }

    result
}