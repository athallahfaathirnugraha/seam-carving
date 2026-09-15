use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn seam_carve(
  bytes: Vec<u8>,
  width: usize,
  height: usize,
  target_width: usize,
  target_height: usize,
) -> Vec<u8> {
  bytes
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
  Ok(())
}
