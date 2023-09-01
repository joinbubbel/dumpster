use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InUploadBase64 {
    pub class_name: String,
    pub base64_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResUploadBase64 {
    pub object_name: Option<String>,
    pub error: Option<UploadBase64Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UploadBase64Error {
    DataCorrupt,
    DataConstraint,
    InvalidBase64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InUploadLooseBase64 {
    pub file_name: String,
    pub base64_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResUploadLooseBase64 {
    pub object_name: Option<String>,
    pub error: Option<UploadBase64Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UploadLooseBase64Error {
    DataConstraint,
    InvalidBase64,
}
