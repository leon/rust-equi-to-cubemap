use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, Rgba};
use std::f32::consts::PI;
use std::fmt::{self, Display};
use FaceOrientation::*;

#[derive(Debug)]
pub enum FaceOrientation { PZ, NZ, PX, NX, PY, NY, }
impl FaceOrientation {
  pub fn faces() -> [FaceOrientation; 6] { [PZ, NZ, PX, NX, PY, NY] }
}
impl Display for FaceOrientation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
 }
}

pub struct Vec3 (pub f32, pub f32, pub f32);
pub fn cartesian_to_spherical(face_orientation: &FaceOrientation, x: f32, y: f32) -> Vec3 {
  let (x, y, z) = match face_orientation {
    PZ => (-1f32, -x, -y),
    NZ => (1f32, x, -y),
    PX => (x, -1f32, -y),
    NX => (-x, 1f32, -y),
    PY => (-y, -x, 1f32),
    NY => (y, -x, -1f32),
  };

  let rotation = (PI * 180f32) / 180f32;
  let r = (x * x + y * y + z * z).sqrt();
  let lat = (z / r).acos();
  let pi2 = PI * 2f32;
  let lng = (((y.atan2(x) + rotation) % pi2) + pi2) % pi2;

  Vec3(r, lat, lng)
}

pub fn convert_face(face: &FaceOrientation, image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let (source_width, source_height) = image.dimensions();

  let face_width: u32 =  source_width / 4;
  let face_height: u32 = face_width;

  let mut face_image = ImageBuffer::new(face_width, face_height);
  for x in 0..face_width {
    for y in 0..face_height {
      let face_x = (2f32 * (x as f32 + 0.5f32)) / face_width as f32 - 1f32;
      let face_y = (2f32 * (y as f32 + 0.5f32)) / face_height as f32 - 1f32;
      let Vec3(_, lat, lng) = cartesian_to_spherical(face, face_x, face_y);

      let source_x = ((source_width as f32 * lng) / PI / 2f32 - 0.5f32) as u32;
      let source_y = ((source_height as f32 * lat) / PI - 0.5f32) as u32;

      unsafe {
        let p = image.unsafe_get_pixel(source_x, source_y);
        face_image.unsafe_put_pixel(x, y, p);
      }
    }
  }
  face_image
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::Path;

  #[test]
  fn test_render_face() {

    let root_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    let test_name = "test3072";
    let image = image::open(root_path.join(format!("assets/{}.jpg", test_name))).unwrap();

    for face in FaceOrientation::faces() {
      let face_image = convert_face(&face, &image);
      face_image.save(root_path.join(format!("out/{}_{}.jpg", test_name, face.to_string().to_lowercase()))).unwrap();
      println!("exported {}", face);
    }
  }
}