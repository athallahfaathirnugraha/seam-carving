type Pixel = [u8; 4];

pub fn image_intensity(pixels: Vec<Pixel>) -> Vec<f64> {
  pixels
    .iter()
    .map(|p| *p.iter().max().unwrap() as f64 / 2.0 + *p.iter().min().unwrap() as f64 / 2.0)
    .collect()
}
