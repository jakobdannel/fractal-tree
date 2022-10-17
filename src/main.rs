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
}

fn main() {
    let args = Args::from_args();

    let mut img = RgbImage::new(args.width, args.height);

    draw_line(&mut img, 0, 0, args.width as i32 - 1, args.height as i32 - 1);

    img.save("./output/output.png").expect("Writing image");
}

fn draw_line(img: &mut RgbImage, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
    for (x, y) in Bresenham::new((x_start, y_start), (x_end, y_end)) {
        img.put_pixel(x as u32, y as u32, Rgb([255,255,255]));
    }
}