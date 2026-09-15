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
  console::log_1(&JsValue::from_str("hi"));
  vec![]
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
  Ok(())
}
