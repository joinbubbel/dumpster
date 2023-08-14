mod class;
mod executor;
mod fs;
mod ops;

#[cfg(test)]
mod tests;

pub use class::{Class, Pipe};
pub use fs::{mock_fs, tokio_fs, FileSystem, FileSystemError};
pub use ops::{Operation, OperationIncoming, OperationOutgoing};

//  TODO Proper error handling.
//  TODO Operation params.
//  TODO Class type filters.
