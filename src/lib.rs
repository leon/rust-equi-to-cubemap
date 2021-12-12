use image::{GenericImageView, ImageBuffer, DynamicImage, Rgba};
use std::f32::consts::PI;
use std::fmt::{self, Debug, Display};

pub struct Vec3 (pub f32, pub f32, pub f32);
impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vec3(x, y, z)
  }

  pub fn zero() -> Self {
    Vec3(0f32, 0f32, 0f32)
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32) {
    self.0 = x;
    self.1 = y;
    self.2 = z;
  }
}

#[derive(Debug)]
pub enum FaceOrientation {
  PZ,
  NZ,
  PX,
  NX,
  PY,
  NY,
}
// impl IntoIterator for FaceOrientation {
//   type Item = FaceOrientation;
//   type IntoIter = std::vec::IntoIter<Self::Item>;

//   fn into_iter(self) -> Self::IntoIter {
//     vec![FaceOrientation::PZ, FaceOrientation::NZ, FaceOrientation::PX, FaceOrientation::NX, FaceOrientation::PY, FaceOrientation::NY]
//   }
// }
impl Display for FaceOrientation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
 }
}

pub fn face_orientation(face_orientation: &FaceOrientation, x: f32, y: f32, cube: &mut Vec3) {
  match face_orientation {
    FaceOrientation::PZ => cube.set(-1f32, -x, -y),
    FaceOrientation::NZ => cube.set(1f32, x, -y),
    FaceOrientation::PX => cube.set(x, -1f32, -y),
    FaceOrientation::NX => cube.set(-x, 1f32, -y),
    FaceOrientation::PY => cube.set(-y, -x, 1f32),
    FaceOrientation::NY => cube.set(y, -x, -1f32),
  }
}

pub fn cartesian_to_spherical(cube: &Vec3) -> Vec3 {
  let Vec3(x, y, z) = cube;
  let rotation = (PI * 180f32) / 180f32;
  let r = (x * x + y * y + z * z).sqrt();
  let lat = (z / r).acos();
  let pi2 = PI * 2f32;
  let lng = (((y.atan2(*x) + rotation) % pi2) + pi2) % pi2;

  Vec3::new(r, lat, lng)
}

pub fn convert_face(face: &FaceOrientation, image: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  // let max_width: u32 = 1536;
  let (source_width, source_height) = image.dimensions();

  let face_width: u32 =  source_width / 4; // std::cmp::max(max_width, source_width / 4);
  let face_height: u32 = face_width;

  let mut cube = Vec3::zero();
  let face_image = ImageBuffer::from_fn(face_width, face_height, |x, y| {

    let face_x = (2f32 * (x as f32 + 0.5f32)) / face_width as f32 - 1f32;
    let face_y = (2f32 * (y as f32 + 0.5f32)) / face_height as f32 - 1f32;
    face_orientation(face, face_x, face_y, &mut cube);
    let Vec3(_, lat, lng) = cartesian_to_spherical(&cube);

    let source_x = (source_width as f32 * lng) / PI / 2f32 - 0.5f32;
    let source_y = (source_height as f32 * lat) / PI - 0.5f32;

    image.get_pixel(source_x as u32, source_y as u32)
  });

  face_image
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_render_face() {

    let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let test_name = "test3072";
    let image = image::open(root_path.join(format!("assets/{}.jpg", test_name))).unwrap();

    for face in [FaceOrientation::PZ, FaceOrientation::NZ, FaceOrientation::PX, FaceOrientation::NX, FaceOrientation::PY, FaceOrientation::NY] {
      let face_image = convert_face(&face, &image);
      face_image.save(root_path.join(format!("out/{}_{}.jpg", test_name, face.to_string().to_lowercase()))).unwrap();
    }
  }
}