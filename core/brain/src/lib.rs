use image::{ImageBuffer, Rgba};

type Pixel = [u8; 4];

pub struct FloatImg {
  pub data: Vec<[f64; 3]>,
  pub width: u32,
  pub height: u32,
}

impl FloatImg {
  pub fn add(a: &FloatImg, b: &FloatImg) -> Result<FloatImg, String> {
    if a.width != b.width || a.height != b.height {
      return Err("a and b have different dimensions".to_string());
    }

    let mut data = vec![[0.0; 3]; a.data.len()];

    for i in 0..a.data.len() {
      data[i][0] = a.data[i][0] + b.data[i][0];
      data[i][1] = a.data[i][1] + b.data[i][1];
      data[i][2] = a.data[i][2] + b.data[i][2];
    }

    Ok(FloatImg {
      data,
      width: a.width,
      height: a.height,
    })
  }

  pub fn clamped(self: FloatImg) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut u8_vec = vec![];

    for pixel in &self.data {
      u8_vec.push(pixel[0] as u8);
      u8_vec.push(pixel[1] as u8);
      u8_vec.push(pixel[2] as u8);
      u8_vec.push(255);
    }

    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
      ImageBuffer::from_vec(self.width, self.height, u8_vec).unwrap();

    img_buffer
  }

  /// Normalized, but the min is never bigger than 0 and the max is never less than 255
  pub fn normalized_255(mut self: FloatImg) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut min = f64::INFINITY;
    let mut max = -f64::INFINITY;

    for pixel in &self.data {
      if pixel[0] < min {
        min = pixel[0];
      }

      if pixel[1] < min {
        min = pixel[1];
      }

      if pixel[2] < min {
        min = pixel[2];
      }

      if pixel[0] > max {
        max = pixel[0];
      }

      if pixel[1] > max {
        max = pixel[1];
      }

      if pixel[2] > max {
        max = pixel[2];
      }
    }

    if min > 0.0 {
      min = 0.0;
    }

    if max < 255.0 {
      max = 255.0;
    }

    for pixel in &mut self.data {
      let normalized = [
        (pixel[0] - min) / (max - min) * 255.0,
        (pixel[1] - min) / (max - min) * 255.0,
        (pixel[2] - min) / (max - min) * 255.0,
      ];

      *pixel = normalized;
    }

    let mut u8_vec = vec![];

    for pixel in &self.data {
      u8_vec.push(pixel[0] as u8);
      u8_vec.push(pixel[1] as u8);
      u8_vec.push(pixel[2] as u8);
      u8_vec.push(255);
    }

    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
      ImageBuffer::from_vec(self.width, self.height, u8_vec).unwrap();

    img_buffer
  }

  pub fn normalized(mut self: FloatImg) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut min = f64::INFINITY;
    let mut max = -f64::INFINITY;

    for pixel in &self.data {
      if pixel[0] < min {
        min = pixel[0];
      }

      if pixel[1] < min {
        min = pixel[1];
      }

      if pixel[2] < min {
        min = pixel[2];
      }

      if pixel[0] > max {
        max = pixel[0];
      }

      if pixel[1] > max {
        max = pixel[1];
      }

      if pixel[2] > max {
        max = pixel[2];
      }
    }

    for pixel in &mut self.data {
      let normalized = [
        (pixel[0] - min) / (max - min) * 255.0,
        (pixel[1] - min) / (max - min) * 255.0,
        (pixel[2] - min) / (max - min) * 255.0,
      ];

      *pixel = normalized;
    }

    let mut u8_vec = vec![];

    for pixel in &self.data {
      u8_vec.push(pixel[0] as u8);
      u8_vec.push(pixel[1] as u8);
      u8_vec.push(pixel[2] as u8);
      u8_vec.push(255);
    }

    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
      ImageBuffer::from_vec(self.width, self.height, u8_vec).unwrap();

    img_buffer
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
