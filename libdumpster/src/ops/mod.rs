//! Synchronous operations to be performed on inputs.
//!
//! ```
//! Check Size Constraints
//! |
//! JPEG to PNG conversion
//! |
//! Store => "{name}.png"
//! |
//! PNG to WEBM conversion
//! |
//! Store => "{name}.webm"
//! ```
//!

mod image;

pub use self::image::*;

pub trait Operation {
    fn incoming(&self, bytes: Vec<u8>) -> Result<Vec<u8>, OperationReject>;
}

pub enum OperationReject {
    DataCorrupt,
    DataConstraint,
}
