use image::{ImageBuffer, Rgba};

type Pixel = [u8; 4];
type FloatImg = (Vec<[f64; 3]>, u32, u32);

pub fn float_img_to_img(float_img: FloatImg) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let mut unwrapped_img = vec![];

  for pixel in &float_img.0 {
    unwrapped_img.push(pixel[0]);
    unwrapped_img.push(pixel[1]);
    unwrapped_img.push(pixel[2]);
  }

  let min = unwrapped_img.iter().fold(f64::INFINITY, |a, &b| a.min(b));
  let max = unwrapped_img.iter().fold(-f64::INFINITY, |a, &b| a.max(b));

  let normalized = |x| ((x / (max - min) - min / (max - min)) * 255.0) as u8;
  let unwrapped_normalized: Vec<u8> = unwrapped_img.iter().map(normalized).collect();

  let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
    float_img.1,
    float_img.2,
    vec![0; (float_img.1 * float_img.2 * 4) as usize],
  )
  .unwrap();

  for y in 0..result.height() {
    for x in 0..result.width() {
      result[(x, y)].0[0] = unwrapped_normalized[(y * float_img.2 + float_img.1 * 3) as usize];
      result[(x, y)].0[1] = unwrapped_normalized[(y * float_img.2 + float_img.1 * 3 + 1) as usize];
      result[(x, y)].0[2] = unwrapped_normalized[(y * float_img.2 + float_img.1 * 3 + 2) as usize];
    }
  }

  result
}

pub fn convolute(img: ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: [f64; 9]) -> FloatImg {
  let mut copy = vec![[0.0; 3]; (img.width() * img.height()) as usize];

  for y in 1..img.height() - 1 {
    for x in 1..img.width() - 1 {
      let pixel_index = (y * img.height() + x) as usize;

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

  (copy, img.width(), img.height())
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
