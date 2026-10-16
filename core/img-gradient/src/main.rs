use std::env;

use brain::pixel_intensity;
use image::ImageReader;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = args[1].clone();
  println!("Opening '{}'", path);

  let img = ImageReader::open(path)
    .expect("Cannot open file")
    .decode()
    .expect("Cannot decode file");

  let mut processed_img = img.clone().into_rgba8();

  for pixel in processed_img.pixels_mut() {
    let intensity = pixel_intensity(pixel.0);
    pixel.0 = [intensity as u8; 4];
  }

  processed_img
    .save("converted.png")
    .expect("Cannot save image");
}
