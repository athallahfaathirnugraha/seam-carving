use image::{ImageBuffer, Rgba};

pub mod energy_img;
pub mod float_img;
pub mod seam;

use crate::{energy_img::EnergyImg, float_img::FloatImg, seam::Seam};

pub type Pixel = [u8; 4];

#[derive(Copy, Clone)]
pub enum Direction {
  Horizontal,
  Vertical,
}

pub fn img_minimal_cost_seam(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Seam {
  let energy_img = gradient_magnitude(img);
  let cumulative_energy_img = energy_img.cumulative_minimum_energy(Direction::Vertical);
  let minimal_cost_seam = cumulative_energy_img.minimal_cost_seam();
  minimal_cost_seam
}

pub fn draw_seam(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, seam: Seam, direction: Direction) {
  match direction {
    Direction::Horizontal => todo!(),
    Direction::Vertical => {
      for y in 0..img.height() {
        let x = seam.mapping[y as usize];
        img[(x as u32, y as u32)] = Rgba::<u8>([255, 0, 0, 255]);
      }
    }
  }
}

pub fn convolute_3x1(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: [f64; 3]) -> FloatImg {
  let mut copy = vec![[0.0; 3]; (img.width() * img.height()) as usize];

  for y in 1..img.height() - 1 {
    for x in 0..img.width() {
      let pixel_index = (y * img.width() + x) as usize;

      for i in 0..3 {
        copy[pixel_index][i] += img[(x, y - 1)].0[i] as f64 * kernel[0];
        copy[pixel_index][i] += img[(x, y + 0)].0[i] as f64 * kernel[1];
        copy[pixel_index][i] += img[(x, y + 1)].0[i] as f64 * kernel[2];
      }
    }
  }

  FloatImg {
    data: copy,
    width: img.width(),
    height: img.height(),
  }
}

pub fn convolute_1x3(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: [f64; 3]) -> FloatImg {
  let mut copy = vec![[0.0; 3]; (img.width() * img.height()) as usize];

  for y in 0..img.height() {
    for x in 1..img.width() - 1 {
      let pixel_index = (y * img.width() + x) as usize;

      for i in 0..3 {
        copy[pixel_index][i] += img[(x - 1, y)].0[i] as f64 * kernel[0];
        copy[pixel_index][i] += img[(x + 0, y)].0[i] as f64 * kernel[1];
        copy[pixel_index][i] += img[(x + 1, y)].0[i] as f64 * kernel[2];
      }
    }
  }

  FloatImg {
    data: copy,
    width: img.width(),
    height: img.height(),
  }
}

pub fn gradient_magnitude(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> EnergyImg {
  let intensity_img = to_intensity_img(img);

  EnergyImg::from_float_img(
    FloatImg::add(
      &convolute_3x1(&intensity_img, [-1.0, 0.0, 1.0]),
      &convolute_1x3(&intensity_img, [-1.0, 0.0, 1.0]),
    )
    .unwrap(),
  )
}

pub fn convolute(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: [f64; 9]) -> FloatImg {
  let mut copy = vec![[0.0; 3]; (img.width() * img.height()) as usize];

  for y in 1..img.height() - 1 {
    for x in 1..img.width() - 1 {
      let pixel_index = (y * img.width() + x) as usize;

      for i in 0..3 {
        copy[pixel_index][i] += img[(x - 1, y - 1)].0[i] as f64 * kernel[0];
        copy[pixel_index][i] += img[(x + 0, y - 1)].0[i] as f64 * kernel[1];
        copy[pixel_index][i] += img[(x + 1, y - 1)].0[i] as f64 * kernel[2];
        copy[pixel_index][i] += img[(x - 1, y + 0)].0[i] as f64 * kernel[3];
        copy[pixel_index][i] += img[(x + 0, y + 0)].0[i] as f64 * kernel[4];
        copy[pixel_index][i] += img[(x + 1, y + 0)].0[i] as f64 * kernel[5];
        copy[pixel_index][i] += img[(x - 1, y + 1)].0[i] as f64 * kernel[6];
        copy[pixel_index][i] += img[(x + 0, y + 1)].0[i] as f64 * kernel[7];
        copy[pixel_index][i] += img[(x + 1, y + 1)].0[i] as f64 * kernel[8];
      }
    }
  }

  FloatImg {
    data: copy,
    width: img.width(),
    height: img.height(),
  }
}

pub fn to_intensity_img(mut img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  for pixel in img.pixels_mut() {
    let intensity = pixel_intensity(pixel.0);
    pixel.0 = [intensity as u8; 4];
  }

  img
}

pub fn pixel_intensity(pixel: Pixel) -> f64 {
  *pixel.iter().max().unwrap() as f64 / 2.0 + *pixel.iter().min().unwrap() as f64 / 2.0
}

pub fn image_intensity(pixels: Vec<Pixel>) -> Vec<f64> {
  pixels
    .iter()
    .map(|p| *p.iter().max().unwrap() as f64 / 2.0 + *p.iter().min().unwrap() as f64 / 2.0)
    .collect()
}
