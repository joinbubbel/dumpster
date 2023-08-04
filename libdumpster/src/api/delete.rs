use super::*;

pub enum DeleteError {
    InsecurePath,
}

pub fn delete(state: &State, path: &str) -> Result<(), DeleteError> {
    let serve_path = state.mount_dir.clone() + path;
    let stat_path = state.mount_dir.clone() + path + MOUNT_STAT_EXT;

    if !check_path(state, &serve_path) {
        Err(DeleteError::InsecurePath)?;
    }
    if !check_path(state, &stat_path) {
        Err(DeleteError::InsecurePath)?;
    }

    let _ = fs::remove_file(path);
    let _ = fs::remove_file(path);

    Ok(())
}
