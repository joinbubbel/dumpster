use super::*;

mod image_size;

#[derive(Serialize, Deserialize, Debug)]
pub enum Constraint {
    ImageSize(image_size::ImageSize),
}
