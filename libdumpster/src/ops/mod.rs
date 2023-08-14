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
#[derive(Debug, Clone)]
pub struct Operation {
    pub incoming: OperationIncoming,
    pub outgoing: OperationOutgoing,
}

pub type OperationIncoming = fn(&str, bytes: Vec<u8>) -> Option<Vec<u8>>;
pub type OperationOutgoing = fn(bytes: Vec<u8>) -> Option<Vec<u8>>;

#[inline]
pub fn no_op(bytes: Vec<u8>) -> Option<Vec<u8>> {
    Some(bytes)
}
