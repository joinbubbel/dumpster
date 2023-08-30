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

mod incoming;

pub use incoming::*;

pub trait Operation {
    fn incoming(&self, data_name: &str, bytes: Vec<u8>) -> Option<Vec<u8>>;
    fn outgoing(&self, bytes: Vec<u8>) -> Option<Vec<u8>>;
}
