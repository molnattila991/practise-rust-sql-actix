mod controllers;

use actix_web::{App, HttpServer};
use controllers::product_controller::{say_hello, get_list, get_list2};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(say_hello)
            .service(get_list)
            .service(get_list2)
    })
    .bind("0.0.0.0:12080")?
    .run()
    .await
}