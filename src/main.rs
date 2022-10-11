use image::{Rgb, RgbImage};

fn main() {
    let img = RgbImage::new(500,500);
    
    img.save("./output/output.png").expect("Writing image");
}
