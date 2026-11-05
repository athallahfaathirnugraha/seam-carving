use image::{ImageBuffer, Rgba};

use crate::{Direction, FloatImg, seam::Seam};

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
      let middle = mapping[(y + 1) as usize] as i32;
      let mut min_index = middle as usize;

      for i in middle - 1..=middle + 1 {
        if i < 0 || i > (self.width - 1) as i32 {
          continue;
        }

        if self.get(i as usize, y as usize) < self.get(min_index, y as usize) {
          min_index = i as usize;
        }
      }

      mapping[y as usize] = min_index;
    }

    Seam::new(mapping).unwrap()
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
