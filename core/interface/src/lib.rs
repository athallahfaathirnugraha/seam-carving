use wasm_bindgen::prelude::*;
use web_sys::*;

type Pixel = [u8; 4];

pub fn image_intensity(pixels: Vec<Pixel>) -> Vec<f64> {
  pixels
    .iter()
    .map(|p| *p.iter().max().unwrap() as f64 / 2.0 + *p.iter().min().unwrap() as f64 / 2.0)
    .collect()
}

#[wasm_bindgen]
pub fn seam_carve(
  bytes: Vec<u8>,
  width: usize,
  height: usize,
  target_width: usize,
  target_height: usize,
) -> Vec<u8> {
  let mut pixels = vec![];

  assert!(bytes.len() % 4 == 0);

  for i in 0..bytes.len() / 4 {
    pixels.push([
      bytes[i * 4],
      bytes[i * 4 + 1],
      bytes[i * 4 + 2],
      bytes[i * 4 + 3],
    ]);
  }

  let mut rotated_pixels = vec![[0u8; 4]; pixels.len()];

  for i in 0..rotated_pixels.len() {
    let x = i % width;
    let y = (i - x) / width;

    rotated_pixels[x * height + (height - 1 - y)] = pixels[i];
  }

  let mut pure_bytes = vec![];

  for pixel in rotated_pixels {
    pure_bytes.push(pixel[0]);
    pure_bytes.push(pixel[1]);
    pure_bytes.push(pixel[2]);
    pure_bytes.push(pixel[3]);
  }

  pure_bytes
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn seam_carve_runs() {
    seam_carve(vec![0, 0, 0, 0], 1, 1, 1, 1);
  }
}
