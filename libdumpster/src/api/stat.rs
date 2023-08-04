use super::*;

pub enum StatError {
    InsecurePath,
    FileError { error: io::Error },
    Internal { error: String },
}

pub fn stat(state: &State, path: &str) -> Result<StoredFileStats, StatError> {
    let path = state.mount_dir.clone() + path;

    if !check_path(state, &path) {
        Err(StatError::InsecurePath)?;
    }

    let stored = fs::read_to_string(&path).map_err(|error| StatError::FileError { error })?;

    let stored = ron::from_str(&stored).map_err(|e| StatError::Internal {
        error: e.to_string(),
    })?;

    Ok(stored)
}
