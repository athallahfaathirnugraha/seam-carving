use std::env;

use brain::{convolute, convolute_3x1, to_intensity_img};
use image::{ImageBuffer, ImageReader, Rgba};

struct ImgProcessor {
  processor: Box<dyn Fn(ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl ImgProcessor {
  pub fn new(
    processor: Box<dyn Fn(ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>>,
  ) -> ImgProcessor {
    ImgProcessor { processor }
  }

  pub fn process(&self, args: Vec<String>) {
    let path = args[0].clone();
    println!("Opening '{}'", path);

    let img = ImageReader::open(path)
      .expect("Cannot open file")
      .decode()
      .expect("Cannot decode file");

    let processed_img = (self.processor)(img.into_rgba8());

    processed_img
      .save("converted.png")
      .expect("Cannot save image");

    println!("Wrote to converted.png");
  }
}

fn main() {
  let img_processors = [
    ImgProcessor::new(Box::new(|img| {
      convolute(img, [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0]).normalized()
    })),
    ImgProcessor::new(Box::new(|img| to_intensity_img(img))),
    ImgProcessor::new(Box::new(|img| {
      convolute_3x1(img, [-1.0, 0.0, 1.0]).clamped()
    })),
  ];

  let args: Vec<String> = env::args().collect();
  let passed_args = args.iter().skip(2).map(|s| s.clone()).collect();

  let img_processor = match args[1].as_str() {
    "intensity" => &img_processors[1],
    "edge" => &img_processors[0],
    "y-gradient" => &img_processors[2],
    arg => {
      println!("argument {arg} not known");
      return;
    }
  };

  img_processor.process(passed_args);
}
