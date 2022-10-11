use image::{Rgb, RgbImage};
extern crate line_drawing;
use line_drawing::Bresenham;

fn main() {
    let mut img = RgbImage::new(500,500);

    draw_line(&mut img, 0, 0, 499, 499);

    img.save("./output/output.png").expect("Writing image");
}

fn draw_line(img: &mut RgbImage, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
    for (x, y) in Bresenham::new((x_start, y_start), (x_end, y_end)) {
        img.put_pixel(x as u32, y as u32, Rgb([255,255,255]));
    }
}