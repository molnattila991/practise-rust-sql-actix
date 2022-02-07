mod controllers;

use actix_web::{App, HttpServer};
use controllers::product_controller::{say_hello, get_list};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(say_hello)
            .service(get_list)
    })
    .bind("0.0.0.0:12080")?
    .run()
    .await
}