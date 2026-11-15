use rand::Rng;

use super::*;

fn random_img(width: usize, height: usize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let mut rng = rand::rng();

  let mut buf = vec![0; width * height * 4];

  for i in 0..width * height * 4 {
    buf[i] = rng.random();
  }

  ImageBuffer::from_vec(width as u32, height as u32, buf).unwrap()
}

/// Tests if reusing the first cumulative minimum energy is equal to recalculating it every seam removal.
#[test]
fn save_first_cumulative_minimum_energy() {
  let img = random_img(200, 200);
}
