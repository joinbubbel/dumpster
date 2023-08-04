//! Mount directory:
//! ```
//! mount/
//!     serve/
//!         foo_class/
//!             0000.jpeg
//!         bar_class/
//!             0000.pdf
//!     stats/
//!         foo_class/
//!             0000.jpeg.ron
//!         bar_class/
//!             0000.pdf.ron
//! ```
use super::*;
use rand::{distributions::Alphanumeric, prelude::*, rngs::OsRng};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const MOUNT_SERVE: &str = "serve";
const MOUNT_STAT: &str = "stats";
const MOUNT_STAT_EXT: &str = ".ron";

mod delete;
mod put;
mod stat;

pub use delete::*;
pub use put::*;
pub use stat::*;

pub struct State {
    mount_dir: String,
}

impl State {
    pub fn new(mount_dir: &str) -> Self {
        State {
            mount_dir: fs::canonicalize(mount_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredFileStats {
    name: String,
    additional: String,
}

/// Ensure that paths like "../../../passwords/" don't work.
fn check_path(state: &State, path: &str) -> bool {
    //  `state.mount_dir` should already be canonicalized.
    let Ok(path) = fs::canonicalize(path) else {
        return false;
    };

    path.to_str().unwrap().chars().count() > state.mount_dir.chars().count()
}

/// Securely generate an alphanumeric token.
fn generate_token_alphanumeric(length: usize) -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
