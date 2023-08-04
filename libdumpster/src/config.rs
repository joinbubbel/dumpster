use super::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    classes: HashMap<String, Class>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mime(String);

#[derive(Serialize, Deserialize, Debug)]
pub enum FileAllow {
    Any,
    Mimes(Vec<Mime>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    allow_files: FileAllow,
    #[serde(default)]
    file_constraints: Vec<Constraint>,
    #[serde(default)]
    file_size_limit_bytes: Option<usize>,
    #[serde(default)]
    global_max_disk_size_bytes: Option<usize>,
}
