mod class;
mod executor;
mod fs;
mod ops;

#[cfg(test)]
mod tests;

pub use class::{Class, Pipe};
pub use executor::Executor;
pub use fs::{mock_fs, tokio_fs, FileSystem, FileSystemError};
pub use ops::*;

//  TODO Proper error handling.
//  TODO Operation params.
//  TODO Class type filters.
