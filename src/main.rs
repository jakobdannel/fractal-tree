use std::f32::consts::PI;

use image::{Rgb, RgbImage};
extern crate line_drawing;
use line_drawing::Bresenham;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(long, short, default_value = "1000")]
    width: u32,
    #[structopt(long, short, default_value = "1000")]
    height: u32,
    #[structopt(long, short, default_value = "6")]
    iterations: u8,
    #[structopt(long, short, default_value = "750.0")]
    length: f32,
    #[structopt(long, short, default_value = "0.6")]
    angle: f32,
}

fn main() {
    let args = Args::from_args();

    let mut img = RgbImage::new(args.width, args.height);

    let x_start = args.width / 2;
    let y_start = args.height - 1;

    draw_tree(
        &mut img,
        args.iterations,
        0.5 * PI,
        args.angle,
        args.length,
        x_start,
        y_start,
    );

    img.save("./output/output.png").expect("Writing image");
}

fn draw_line(img: &mut RgbImage, x_start: u32, y_start: u32, x_end: u32, y_end: u32) {
    for (x, y) in Bresenham::new(
        (x_start as i32, y_start as i32),
        (x_end as i32, y_end as i32),
    ) {
        img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
    }
}

fn draw_tree(
    img: &mut RgbImage,
    iterations: u8,
    init_angle: f32,
    angle: f32,
    mut length: f32,
    x_start: u32,
    y_start: u32,
) {
    length = length * 0.5;
    let x_end: u32 = (x_start as f32 - length * f32::cos(init_angle)) as u32;
    let y_end: u32 = (y_start as f32 - length * f32::sin(init_angle)) as u32;
    draw_line(img, x_start, y_start, x_end, y_end);

    if iterations > 0 {
        draw_tree(img, iterations - 1, init_angle - angle, angle, length, x_end, y_end);
        draw_tree(img, iterations - 1, init_angle + angle, angle, length, x_end, y_end);
    }
}
