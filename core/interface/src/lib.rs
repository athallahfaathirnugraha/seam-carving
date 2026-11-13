use brain::{img_minimal_cost_seam, remove_seam};

use image::{ImageBuffer, Rgba};

use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn seam_carve(bytes: Vec<u8>, width: usize, height: usize, target_width: usize) -> Vec<u8> {
  let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
    ImageBuffer::from_vec(width as u32, height as u32, bytes).unwrap();

  for i in 0..width - target_width {
    let seam = img_minimal_cost_seam(img.clone());
    img = remove_seam(&img, &seam);
  }

  img.into_vec()
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
    seam_carve(vec![0, 0, 0, 0], 1, 1, 1);
  }
}
