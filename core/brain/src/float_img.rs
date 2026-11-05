use image::{ImageBuffer, Rgba};

pub struct FloatImg {
  pub data: Vec<[f64; 3]>,
  pub width: u32,
  pub height: u32,
}

impl FloatImg {
  pub fn get(&self, x: usize, y: usize) -> [f64; 3] {
    self.data[y * self.width as usize + x]
  }

  pub fn sqrt(&self) -> FloatImg {
    let mut data = vec![[0.0; 3]; self.data.len()];

    for i in 0..self.data.len() {
      data[i][0] = self.data[i][0].sqrt();
      data[i][1] = self.data[i][1].sqrt();
      data[i][2] = self.data[i][2].sqrt();
    }

    FloatImg {
      data,
      width: self.width,
      height: self.height,
    }
  }

  pub fn squared(&self) -> FloatImg {
    let mut data = vec![[0.0; 3]; self.data.len()];

    for i in 0..self.data.len() {
      data[i][0] = self.data[i][0] * self.data[i][0];
      data[i][1] = self.data[i][1] * self.data[i][1];
      data[i][2] = self.data[i][2] * self.data[i][2];
    }

    FloatImg {
      data,
      width: self.width,
      height: self.height,
    }
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
