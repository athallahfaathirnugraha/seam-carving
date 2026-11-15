use crate::{Direction, EnergyImg};

#[derive(Debug)]
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

  pub fn mapping(&self) -> &Vec<usize> {
    &self.mapping
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

impl PartialEq for Seam {
  fn eq(&self, other: &Self) -> bool {
    self.mapping == other.mapping
  }
}
