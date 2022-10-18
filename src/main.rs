use std::f32::consts::PI;
use std::fs;

use rand::Rng;

use image::RgbImage;
extern crate line_drawing;
use line_drawing::Bresenham;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(long, short, default_value = "1000")]
    width: u32,
    #[structopt(long, short, default_value = "1000")]
    height: u32,
    #[structopt(long, short, default_value = "11")]
    iterations: u8,
    #[structopt(long, short, default_value = "250.0")]
    length: f32,
    #[structopt(long, short, default_value = "0.6")]
    angle: f32,
    #[structopt(long, short)]
    color: bool,
}

#[derive(Clone, Copy)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(rgb: Rgb) -> Self {
        image::Rgb([rgb.red, rgb.green, rgb.blue])
    }
}

fn main() {
    let args = Args::from_args();

    let mut img = RgbImage::new(args.width, args.height);

    let x_start = args.width / 2;
    let y_start = args.height - 1;

    draw_tree(
        &mut img,
        args.iterations,
        args.iterations,
        0.5 * PI,
        args.angle,
        args.length,
        x_start,
        y_start,
        args.color,
    );
    fs::create_dir_all("output").expect("Creating folder");
    img.save("./output/output.png").expect("Writing image");
}

fn draw_line(img: &mut RgbImage, x_start: u32, y_start: u32, x_end: u32, y_end: u32, color: Rgb) {
    for (x, y) in Bresenham::new(
        (x_start as i32, y_start as i32),
        (x_end as i32, y_end as i32),
    ) {
        img.put_pixel(x as u32, y as u32, color.into());
    }
}

fn draw_tree(
    img: &mut RgbImage,
    init_iterations: u8,
    iterations: u8,
    init_angle: f32,
    angle: f32,
    mut length: f32,
    x_start: u32,
    y_start: u32,
    colorful: bool,
) {
    length = length * 0.7;
    let x_end: u32 = (x_start as f32 - length * f32::cos(init_angle)) as u32;
    let y_end: u32 = (y_start as f32 - length * f32::sin(init_angle)) as u32;
    let color: Rgb;
    if colorful {
        color = hsl_to_rgb(
            (iterations as f32 / init_iterations as f32) - 0.01,
            1.0,
            0.5,
        );
    } else {
        color = hsl_to_rgb(0.0, 0.0, 1.0);
    }
    draw_line(img, x_start, y_start, x_end, y_end, color);

    if iterations > 0 {
        let mut rng = rand::thread_rng();
        draw_tree(
            img,
            iterations,
            iterations - 1,
            init_angle - angle + 0.4 * rng.gen::<f32>() - 0.2,
            angle,
            length,
            x_end,
            y_end,
            colorful,
        );
        draw_tree(
            img,
            iterations,
            iterations - 1,
            init_angle + angle + 0.4 * rng.gen::<f32>() - 0.2,
            angle,
            length,
            x_end,
            y_end,
            colorful,
        );
    }
}

fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> Rgb {
    let c = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
    let h = hue * 6.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = luminance - (c / 2.0);

    let i = h.floor() as usize;
    let mut rgb_table = [c, x, 0.0];
    if i & 1 == 1 {
        rgb_table.swap(0, 1);
    }
    let (r, g, b) = (
        rgb_table[(i / 2) % 3],
        rgb_table[(i / 2 + 1) % 3],
        rgb_table[(i / 2 + 2) % 3],
    );

    Rgb {
        red: ((r + m) * 255.0) as u8,
        green: ((g + m) * 255.0) as u8,
        blue: ((b + m) * 255.0) as u8,
    }
}
