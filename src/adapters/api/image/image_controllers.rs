use actix_web::{post, web, HttpResponse};
use crate::adapters::api::shared::response::ApiResponse;
use crate::domain::entities::{EncodeRequest, DecodeRequest};
use crate::application::usecases::{encode_image_usecase, decode_image_usecase};

#[post("/image/encode")]
pub async fn encode_image_handler(request: web::Json<EncodeRequest>) -> HttpResponse {
    match encode_image_usecase::encode_image(request.into_inner()) {
        Ok(response) => {
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(_) => {
            HttpResponse::NotFound().json(ApiResponse::error("Failed to encode image"))
        }
    }
}

#[post("/image/decode")]
pub async fn decode_image_handler(request: web::Json<DecodeRequest>) -> HttpResponse {
    match decode_image_usecase::decode_image(request.into_inner()) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::success("Image decoded successfully"))
        }
        Err(_) => {
            HttpResponse::NotFound().json(ApiResponse::error("Failed to decode image"))
        }
    }
}