use actix_web::web;

use crate::adapters::api::ping::ping_controller::ping_handler;
use crate::adapters::api::index;
use crate::adapters::api::image::image_controllers::{encode_image_handler, decode_image_handler};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
       .service(index)
       .service(ping_handler)
       .service(encode_image_handler)
       .service(decode_image_handler));
}