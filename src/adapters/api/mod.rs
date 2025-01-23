pub mod ping;
pub mod shared;
pub mod image;

use actix_web::{get, HttpResponse};
use shared::response::ApiResponse;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("Hello, world!"))
}
