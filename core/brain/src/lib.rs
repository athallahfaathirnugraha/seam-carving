use image::{ImageBuffer, Rgba};

type Pixel = [u8; 4];

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
