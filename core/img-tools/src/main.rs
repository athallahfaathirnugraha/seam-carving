use std::env;

use brain::to_intensity_img;
use image::ImageReader;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = args[1].clone();
  println!("Opening '{}'", path);

  let img = ImageReader::open(path)
    .expect("Cannot open file")
    .decode()
    .expect("Cannot decode file");

  let processed_img = to_intensity_img(img.into_rgba8());

  processed_img
    .save("converted.png")
    .expect("Cannot save image");

  println!("Wrote to converted.png");
}
