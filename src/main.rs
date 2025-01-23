mod infrastructure;
mod adapters;
mod domain;
mod application;

use actix_web::{App, HttpServer};
use infrastructure::get_server_address;
use adapters::api::shared::routes::configure_routes;

use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let server_address = get_server_address();
    println!("Server running at http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
    .bind(server_address)?
    .run()
    .await
}