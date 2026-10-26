use std::env;

use brain::{convolute, convolute_3x1, to_intensity_img};
use image::{DynamicImage, ImageReader};

fn open_img(path: String) -> DynamicImage {
  let img = ImageReader::open(path)
    .expect("Cannot open file")
    .decode()
    .expect("Cannot decode file");

  img
}

fn y_gradient(args: Vec<String>) {
  let path = args[0].clone();
  println!("Opening '{}'", path);

  let img = open_img(path);

  let processed_img = convolute_3x1(img.into_rgba8(), [-1.0, 0.0, 1.0]).clamped();

  processed_img
    .save("converted.png")
    .expect("Cannot save image");

  println!("Wrote to converted.png");
}

fn intensity(args: Vec<String>) {
  let path = args[0].clone();
  println!("Opening '{}'", path);

  let img = open_img(path);

  let processed_img = to_intensity_img(img.into_rgba8());

  processed_img
    .save("converted.png")
    .expect("Cannot save image");

  println!("Wrote to converted.png");
}

fn edge(args: Vec<String>) {
  let path = args[0].clone();
  println!("Opening '{}'", path);

  let img = open_img(path);

  let processed_img = convolute(
    img.into_rgba8(),
    [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0],
  )
  .normalized();

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
    "edge" => edge(passed_args),
    "y-gradient" => y_gradient(passed_args),
    arg => println!("argument {arg} not known"),
  }
}
