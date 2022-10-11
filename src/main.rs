use image::{Rgb, RgbImage};
extern crate line_drawing;
use line_drawing::Bresenham;

fn main() {
    let mut img = RgbImage::new(500,500);

    for (x, y) in Bresenham::new((0, 0), (499, 499)) {
        img.put_pixel(x as u32, y as u32, Rgb([255,255,255]));
    }
    img.save("./output/output.png").expect("Writing image");
}
