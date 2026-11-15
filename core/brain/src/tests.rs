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

#[test]
#[ignore]
fn removed_seam_cme_equal_to_recalculated_cme() {
  let img = random_img(200, 200);
  let seam = img_minimal_cost_seam(img.clone());

  let mut removed_seam_cme = e1(&img).cumulative_minimum_energy(Direction::Vertical);
  removed_seam_cme.remove_seam(&seam);

  let recalculated_cme = {
    let removed_seam_img = remove_seam(&img, &seam);
    e1(&removed_seam_img).cumulative_minimum_energy(Direction::Vertical)
  };

  assert_eq!(recalculated_cme, removed_seam_cme);
}

/// Tests if reusing the first cumulative minimum energy is equal to recalculating it every seam removal.
#[test]
#[ignore]
fn save_first_cumulative_minimum_energy() {
  let mut img1 = random_img(200, 200);
  let mut img2 = img1.clone();

  let mut cme = e1(&img2).cumulative_minimum_energy(Direction::Vertical);

  for i in 0..img1.width() - 1 {
    dbg!(i);

    // img1 (recalculate)
    let seam1 = img_minimal_cost_seam(img1.clone());
    img1 = remove_seam(&img1, &seam1);

    // img2 (reuse first cumulative minimum energy)
    let seam2 = cme.minimal_cost_seam();
    img2 = remove_seam(&img2, &seam2);
    cme.remove_seam(&seam2);

    assert_eq!(seam1, seam2);
  }
}
