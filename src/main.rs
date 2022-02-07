mod controllers;

use actix_web::{App, HttpServer};
use controllers::product_controller::{hello, hello2};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello2)
    })
    .bind("0.0.0.0:12080")?
    .run()
    .await
}