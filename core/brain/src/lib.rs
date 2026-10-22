use image::{ImageBuffer, Rgba};

type Pixel = [u8; 4];

pub fn convolute(
  img: ImageBuffer<Rgba<u8>, Vec<u8>>,
  kernel: [f64; 9],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let mut copy: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
    img.width(),
    img.height(),
    vec![0; (img.width() * img.height() * 4) as usize],
  )
  .unwrap();

  for y in 1..img.height() - 1 {
    for x in 1..img.width() - 1 {
      for i in 0..3 {
        let mut val: f64 = 0.0;
        val += img[(x - 1, y - 1)].0[i] as f64 * kernel[0];
        val += img[(x + 0, y - 1)].0[i] as f64 * kernel[1];
        val += img[(x + 1, y - 1)].0[i] as f64 * kernel[2];
        val += img[(x - 1, y + 0)].0[i] as f64 * kernel[3];
        val += img[(x + 0, y + 0)].0[i] as f64 * kernel[4];
        val += img[(x + 1, y + 0)].0[i] as f64 * kernel[5];
        val += img[(x - 1, y + 1)].0[i] as f64 * kernel[6];
        val += img[(x + 0, y + 1)].0[i] as f64 * kernel[7];
        val += img[(x + 1, y + 1)].0[i] as f64 * kernel[8];
        copy[(x, y)].0[i] = val as u8;
      }

      copy[(x, y)].0[3] = 255;
    }
  }

  copy
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
