use std::io;

use brain::{
  Direction, convolute, convolute_1x3, convolute_3x1, draw_seam, float_img::FloatImg,
  gradient_magnitude, img_minimal_cost_seam, remove_seam, to_intensity_img,
};

use image::ImageReader;

use clap::{Parser, ValueEnum, builder::PossibleValue};

#[derive(Debug, Clone)]
enum Subcommand {
  Intensity,
  Edge,
  XGradient,
  YGradient,
  XyGradient,
  AbsXyGradient,
  GradientMagnitude,
  CumulativeMinimumEnergy,
  MinimalCostSeam,
  SeamSteps,
}

impl clap::ValueEnum for Subcommand {
  fn value_variants<'a>() -> &'a [Self] {
    use Subcommand::*;

    &[
      Intensity,
      Edge,
      XGradient,
      YGradient,
      XyGradient,
      AbsXyGradient,
      GradientMagnitude,
      CumulativeMinimumEnergy,
      MinimalCostSeam,
      SeamSteps,
    ]
  }

  fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
    use Subcommand::*;

    match self {
      Intensity => Some(PossibleValue::new("intensity")),
      Edge => Some(PossibleValue::new("edge")),
      XGradient => Some(PossibleValue::new("x-gradient")),
      YGradient => Some(PossibleValue::new("y-gradient")),
      XyGradient => Some(PossibleValue::new("xy-gradient")),
      AbsXyGradient => Some(PossibleValue::new("abs-xy-gradient")),
      GradientMagnitude => Some(PossibleValue::new("gradient-magnitude")),
      CumulativeMinimumEnergy => Some(PossibleValue::new("cumulative-minimum-energy")),
      MinimalCostSeam => Some(PossibleValue::new("minimal-cost-seam")),
      SeamSteps => Some(PossibleValue::new("seam-steps")),
    }
  }
}

#[derive(Parser, Debug)]
struct Args {
  /// The subcommand to run.
  subcommand: Subcommand,
  /// The path of the image to process.
  img_path: String,
}

fn main() -> Result<(), String> {
  use Subcommand::*;

  let args = Args::parse();

  let path = args.img_path.clone();
  println!("Opening '{}'", path);

  let mut img = ImageReader::open(path)
    .expect("Cannot open file")
    .decode()
    .expect("Cannot decode file")
    .into_rgba8();

  match args.subcommand {
    SeamSteps => {
      while let Err(err) = std::fs::create_dir("seam-steps") {
        if err.kind() == io::ErrorKind::AlreadyExists {
          std::fs::remove_dir_all("seam-steps").unwrap();
          continue;
        } else {
          dbg!(err);
          return Err("unknown error".to_string());
        }
      }

      for i in 0..img.width() - 1 {
        let seam = img_minimal_cost_seam(img.clone());
        draw_seam(&mut img, &seam, Direction::Vertical);
        img.save(format!("seam-steps/{}.png", i)).unwrap();
        img = remove_seam(&img, &seam);
      }

      println!("Wrote to seam-steps/");

      return Ok(());
    }
    _ => (),
  }

  let new_img = match args.subcommand {
    Edge => convolute(&img, [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0]).normalized(),
    Intensity => to_intensity_img(img),
    XGradient => convolute_1x3(&img, [-1.0, 0.0, 1.0]).normalized_255(),
    YGradient => convolute_3x1(&img, [-1.0, 0.0, 1.0]).normalized_255(),
    XyGradient => FloatImg::add(
      &convolute_3x1(&img, [-1.0, 0.0, 1.0]),
      &convolute_1x3(&img, [-1.0, 0.0, 1.0]),
    )
    .unwrap()
    .normalized_255(),
    AbsXyGradient => FloatImg::add(
      &convolute_3x1(&img, [-1.0, 0.0, 1.0]),
      &convolute_1x3(&img, [-1.0, 0.0, 1.0]),
    )
    .unwrap()
    .abs()
    .normalized_255(),
    GradientMagnitude => gradient_magnitude(img).abs().normalized_255(),
    CumulativeMinimumEnergy => gradient_magnitude(img)
      .cumulative_minimum_energy(Direction::Vertical)
      .normalized_255(),
    MinimalCostSeam => {
      let seam = img_minimal_cost_seam(img.clone());
      draw_seam(&mut img, &seam, Direction::Vertical);
      img
    }
    SeamSteps => unreachable!(),
  };

  let save_path = args
    .subcommand
    .to_possible_value()
    .unwrap()
    .get_name()
    .to_string()
    + ".png";

  new_img.save(&save_path).expect("Cannot save image");

  println!("Wrote to {}", &save_path);

  Ok(())
}
