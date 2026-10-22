use std::env;

use brain::to_intensity_img;
use image::ImageReader;

fn intensity(args: Vec<String>) {
  let path = args[0].clone();
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

fn main() {
  let args: Vec<String> = env::args().collect();
  let passed_args = args.iter().skip(2).map(|s| s.clone()).collect();

  match args[1].as_str() {
    "intensity" => intensity(passed_args),
    arg => println!("argument {arg} not known"),
  }
}
