use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use base64::Engine;
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct EncodeRequest {
    file_path: String,
}

#[derive(Serialize)]
struct EncodeResponse {
    base64_data: String,
}

#[derive(Deserialize)]
struct DecodeRequest {
    base64_data: String,
    output_path: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/image/encode")]
async fn encode_image(request: web::Json<EncodeRequest>) -> impl Responder {
    match fs::read(&request.file_path) {
        Ok(image_data) => {
            let base64_data = base64::engine::general_purpose::STANDARD.encode(&image_data);
            HttpResponse::Ok().json(EncodeResponse { base64_data })
        }
        Err(_) => {
            HttpResponse::NotFound().body("Failed to read image file")
        }
    }
}

#[post("/image/decode")]
async fn decode_image(request: web::Json<DecodeRequest>) -> impl Responder {
    match base64::engine::general_purpose::STANDARD.decode(&request.base64_data) {
        Ok(image_data) => {
            let image = image::load_from_memory(&image_data).unwrap();
            match image.save(&request.output_path) {
                Ok(_) => {
                    HttpResponse::Ok().body("Image saved successfully")
                }
                Err(_) => {
                    HttpResponse::NotFound().body("Failed to save image")
                }
            }
        }
        Err(_) => {
            HttpResponse::NotFound().body("Failed to decode base64 data")
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(encode_image)
            .service(decode_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
