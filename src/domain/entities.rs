use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncodeRequest {
    pub file_path: String,
}

#[derive(Serialize)]
pub struct EncodeResponse {
    pub base64_data: String,
}

#[derive(Deserialize)]
pub struct DecodeRequest {
    pub base64_data: String,
    pub output_path: String,
}