use std::env;

use brain::{
  Direction, convolute, convolute_1x3, convolute_3x1, float_img::FloatImg, gradient_magnitude,
  to_intensity_img,
};
use image::{ImageBuffer, ImageReader, Rgba};

struct ImgProcessor {
  processor: Box<dyn Fn(ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>>,
  save_path: String,
}

impl ImgProcessor {
  pub fn new(
    processor: Box<dyn Fn(ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>>,
    save_path: String,
  ) -> ImgProcessor {
    ImgProcessor {
      processor,
      save_path,
    }
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
      .save(&self.save_path)
      .expect("Cannot save image");

    println!("Wrote to {}", &self.save_path);
  }
}

fn main() {
  let img_processors = [
    ImgProcessor::new(
      Box::new(|img| {
        convolute(&img, [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0]).normalized()
      }),
      "edge.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| to_intensity_img(img)),
      "intensity.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| convolute_1x3(&img, [-1.0, 0.0, 1.0]).normalized_255()),
      "x-gradient.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| convolute_3x1(&img, [-1.0, 0.0, 1.0]).normalized_255()),
      "y-gradient.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| {
        FloatImg::add(
          &convolute_3x1(&img, [-1.0, 0.0, 1.0]),
          &convolute_1x3(&img, [-1.0, 0.0, 1.0]),
        )
        .unwrap()
        .normalized_255()
      }),
      "xy-gradient.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| {
        FloatImg::add(
          &convolute_3x1(&img, [-1.0, 0.0, 1.0]),
          &convolute_1x3(&img, [-1.0, 0.0, 1.0]),
        )
        .unwrap()
        .abs()
        .normalized_255()
      }),
      "abs-xy-gradient.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| gradient_magnitude(img).abs().normalized_255()),
      "gradient-magnitude.png".to_string(),
    ),
    ImgProcessor::new(
      Box::new(|img| {
        gradient_magnitude(img)
          .cumulative_minimum_energy(Direction::Vertical)
          .normalized_255()
      }),
      "cumulative-minimum-energy.png".to_string(),
    ),
  ];

  let args: Vec<String> = env::args().collect();
  let passed_args = args.iter().skip(2).map(|s| s.clone()).collect();

  let img_processor = match args[1].as_str() {
    "intensity" => &img_processors[1],
    "edge" => &img_processors[0],
    "x-gradient" => &img_processors[2],
    "y-gradient" => &img_processors[3],
    "xy-gradient" => &img_processors[4],
    "abs-xy-gradient" => &img_processors[5],
    "gradient-magnitude" => &img_processors[6],
    "cumulative-minimum-energy" => &img_processors[7],
    arg => {
      println!("argument {arg} not known");
      return;
    }
  };

  img_processor.process(passed_args);
}
