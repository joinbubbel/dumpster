use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
    min_image_size_x: usize,
    min_image_size_y: usize,

    max_image_size_x: usize,
    max_image_size_y: usize,
}
