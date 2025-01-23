use crate::adapters::spi::file_system::file_repository;
use crate::domain::entities::DecodeRequest;
use base64::Engine;

pub fn decode_image(request: DecodeRequest) -> Result<(), String> {
    match base64::engine::general_purpose::STANDARD.decode(&request.base64_data) {
        Ok(image_data) => {
            match file_repository::save_file(&request.output_path, &image_data) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to save image".to_string())
            }
        }
        Err(_) => {
            Err("Failed to decode base64 data".to_string())
        }
    }
}