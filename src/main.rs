use equi_to_cubemap::*;
use std::path::PathBuf;

// fn get_nth_arg(n: usize) -> String {
//   std::env::args().nth(n).unwrap()
// }

fn main() {
    // let image_path = get_nth_arg(1);
    let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let test_name = "test3072";
    let image = image::open(root_path.join(format!("assets/{}.jpg", test_name))).unwrap();

    for face in [FaceOrientation::PZ, FaceOrientation::NZ, FaceOrientation::PX, FaceOrientation::NX, FaceOrientation::PY, FaceOrientation::NY] {
      let face_image = convert_face(&face, &image);
      face_image.save(root_path.join(format!("out/{}_{}.jpg", test_name, face.to_string().to_lowercase()))).unwrap();
    }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_render_face() {
    

//     let cube = Vec3::new(0, 0, 0);

//     assert_eq!(cube.x, 0);
//     // let cube = face_orientation(FaceOrientation::NX, 1, 1, cube);
//     // let res = cartesian_to_spherical()
//   }
// }