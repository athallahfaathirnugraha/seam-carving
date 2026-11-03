use image::{ImageBuffer, Rgba};

pub type Pixel = [u8; 4];

#[derive(Copy, Clone)]
pub enum Direction {
  Horizontal,
  Vertical,
}

pub struct Seam {
  mapping: Vec<usize>,
}

impl Seam {
  pub fn new(mapping: Vec<usize>) -> Result<Seam, String> {
    for i in 1..mapping.len() {
      if mapping[i].abs_diff(mapping[i - 1]) > 1 {
        return Err("invalid k".to_string());
      }
    }

    Ok(Seam { mapping })
  }

  pub fn cost(&self, energy: EnergyImg, direction: Direction) -> Result<f64, String> {
    match direction {
      Direction::Horizontal => todo!(),
      Direction::Vertical => {
        if self.mapping.len() != energy.height as usize {
          return Err("mapping size is not equal to image height".to_string());
        }

        let mut sum = 0.0;

        for y in 0..self.mapping.len() {
          let x = self.mapping[y];
          sum += energy.get(x, y);
        }

        Ok(sum)
      }
    }
  }
}

pub struct EnergyImg {
  pub data: Vec<f64>,
  pub width: u32,
  pub height: u32,
}

impl EnergyImg {
  pub fn empty(width: u32, height: u32) -> EnergyImg {
    EnergyImg {
      data: vec![0.0; (width * height) as usize],
      width,
      height,
    }
  }

  pub fn get(&self, x: usize, y: usize) -> f64 {
    self.data[y * self.width as usize + x]
  }

  pub fn from_float_img(float_img: FloatImg) -> EnergyImg {
    EnergyImg {
      data: float_img.data.into_iter().map(|x| x[0]).collect(),
      width: float_img.width,
      height: float_img.height,
    }
  }

  pub fn abs(&self) -> EnergyImg {
    let mut data = self.data.clone();

    for pixel in &mut data {
      *pixel = pixel.abs();
    }

    EnergyImg {
      data,
      width: self.width,
      height: self.height,
    }
  }

  /// `self` is the cumulative minimum energy image.
  pub fn minimal_cost_seam(&self) -> Seam {
    let mut mapping = vec![0; self.height as usize];

    let mut min_index = 0;

    for i in 1..self.width {
      if self.get(i as usize, (self.height - 1) as usize)
        < self.get(min_index, (self.height - 1) as usize)
      {
        min_index = i as usize;
      }
    }

    mapping[(self.height - 1) as usize] = min_index;

    for y in (0..self.height - 1).rev() {
      let middle = mapping[(y + 1) as usize];
      let mut min_index = 0;

      for i in middle - 1..=middle + 1 {
        if self.get(i as usize, y as usize) < self.get(min_index, y as usize) {
          min_index = i as usize;
        }
      }

      mapping[y as usize] = min_index;
    }

    Seam { mapping }
  }

  pub fn cumulative_minimum_energy(&self, direction: Direction) -> EnergyImg {
    fn M(
      x: u32,
      y: u32,
      energy: &EnergyImg,
      cache: &mut Vec<Option<f64>>,
      direction: Direction,
    ) -> f64 {
      if let Some(value) = cache[(y * energy.width + x) as usize] {
        value
      } else {
        match direction {
          Direction::Horizontal => todo!(),
          Direction::Vertical => {
            let value = if y == 0 {
              energy.get(x as usize, y as usize)
            } else {
              energy.get(x as usize, y as usize)
                + f64::min(
                  if x > 0 {
                    M(x - 1, y - 1, energy, cache, direction)
                  } else {
                    0.0
                  },
                  f64::min(
                    M(x, y - 1, energy, cache, direction),
                    if x < energy.width - 1 {
                      M(x + 1, y - 1, energy, cache, direction)
                    } else {
                      0.0
                    },
                  ),
                )
            };

            cache[(y * energy.width + x) as usize] = Some(value);
            value
          }
        }
      }
    }

    let mut cache = vec![None; (self.width * self.height) as usize];

    match direction {
      Direction::Horizontal => todo!(),
      Direction::Vertical => {
        for y in 1..self.height {
          for x in 0..self.width {
            M(x, y, self, &mut cache, direction);
          }
        }

        EnergyImg {
          data: cache.into_iter().map(|x| x.unwrap()).collect(),
          width: self.width,
          height: self.height,
        }
      }
    }
  }

  /// Normalized, but the min is never bigger than 0 and the max is never less than 255
  pub fn normalized_255(mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut min = f64::INFINITY;
    let mut max = -f64::INFINITY;

    for pixel in &self.data {
      if *pixel < min {
        min = *pixel;
      }

      if *pixel > max {
        max = *pixel;
      }
    }

    if min > 0.0 {
      min = 0.0;
    }

    if max < 255.0 {
      max = 255.0;
    }

    for pixel in &mut self.data {
      let normalized = (*pixel - min) / (max - min) * 255.0;

      *pixel = normalized;
    }

    let mut u8_vec = vec![];

    for pixel in &self.data {
      u8_vec.push(*pixel as u8);
      u8_vec.push(*pixel as u8);
      u8_vec.push(*pixel as u8);
      u8_vec.push(255);
    }

    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
      ImageBuffer::from_vec(self.width, self.height, u8_vec).unwrap();

    img_buffer
  }
}

pub struct FloatImg {
  pub data: Vec<[f64; 3]>,
  pub width: u32,
  pub height: u32,
}

impl FloatImg {
  pub fn get(&self, x: usize, y: usize) -> [f64; 3] {
    self.data[y * self.width as usize + x]
  }

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

  pub fn abs(self: &FloatImg) -> FloatImg {
    let mut data = self.data.clone();

    for pixel in &mut data {
      pixel[0] = pixel[0].abs();
      pixel[1] = pixel[1].abs();
      pixel[2] = pixel[2].abs();
    }

    FloatImg {
      data,
      width: self.width,
      height: self.height,
    }
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
