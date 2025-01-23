use crate::adapters::spi::file_system::file_repository;
use crate::domain::entities::{EncodeRequest, EncodeResponse};
use base64::Engine;

pub fn encode_image(request: EncodeRequest) -> Result<EncodeResponse, String> {
    match file_repository::read_file(&request.file_path){
        Ok(image_data) => {
            let base64_data = base64::engine::general_purpose::STANDARD.encode(&image_data);
            Ok(EncodeResponse { base64_data })
        }
        Err(_) => {
            Err("Failed to read image file".to_string())
        }
    }
}
