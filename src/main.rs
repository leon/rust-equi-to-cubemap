use equi_to_cubemap::*;
use std::path::Path;

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

fn main() {
  let arg = get_nth_arg(1);
  let image_path = Path::new(&arg);
  let filename = image_path.file_name().unwrap().to_str().unwrap();
  let out_path = image_path.parent().unwrap();

  let image = image::open(image_path).unwrap();

  for face in FaceOrientation::faces() {
    let face_image = convert_face(&face, &image);
    face_image.save(out_path.join(format!("{}_{}.jpg", filename, face.to_string().to_lowercase()))).unwrap();
  }
}
