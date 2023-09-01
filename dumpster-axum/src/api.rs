use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InUpload {
    pub class_name: String,
    pub base64_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResUpload {
    pub object_name: Option<String>,
    pub error: Option<ResError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResError {
    DataCorrupt,
    DataConstraint,
    InvalidBase64,
}
