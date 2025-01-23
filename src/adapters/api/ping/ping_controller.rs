use actix_web::{get, HttpResponse};

use crate::adapters::api::shared::response::ApiResponse;

#[get("/ping")]
pub async fn ping_handler() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("Pong!"))
}

