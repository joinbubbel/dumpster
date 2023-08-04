use super::*;

const FILE_TOKEN_LENGTH: usize = 64;

pub enum PutError {
    FileError { error: io::Error },
    InsecurePath,
}

pub fn put(
    state: &State,
    file_bytes: &[u8],
    file_name: &Path,
    name: &str,
    additional: &str,
) -> Result<String, PutError> {
    let token = generate_token_alphanumeric(FILE_TOKEN_LENGTH);

    let file_name = PathBuf::from(file_name);
    let ext = file_name.extension().unwrap().to_str().unwrap();

    let mut serve_path = PathBuf::from(&state.mount_dir);
    serve_path.push(MOUNT_SERVE);
    serve_path.push(&token);
    serve_path.set_extension(ext);

    let mut stat_path = PathBuf::from(&state.mount_dir);
    stat_path.push(MOUNT_STAT);
    stat_path.push(&token);
    serve_path.set_extension(ext.to_owned() + MOUNT_STAT_EXT);

    if !check_path(state, serve_path.to_str().unwrap()) {
        Err(PutError::InsecurePath)?;
    }
    if !check_path(state, stat_path.to_str().unwrap()) {
        Err(PutError::InsecurePath)?;
    }

    let stats = StoredFileStats {
        name: name.to_owned(),
        additional: additional.to_owned(),
    };
    let stats = ron::to_string(&stats).unwrap();

    fs::write(&serve_path, file_bytes).map_err(|error| PutError::FileError { error })?;
    fs::write(stat_path, stats).map_err(|error| PutError::FileError { error })?;

    Ok(serve_path.to_str().unwrap().to_string())
}
