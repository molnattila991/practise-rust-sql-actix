use mysql::*;
use mysql::prelude::*;
// use chrono::prelude::*; //For date and time
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: u64,
    code: String,
    price: f32,
    product_name: String,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[get("/jsonList")]
async fn hello2() -> impl Responder {
    // let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@172.21.192.1:3306/shop-api";
    let url = "mysql://shop-api:shop-api@db:3306/shop-api";
    
    let opt = Opts::from_url(url);
    let pool = Pool::new(opt.unwrap()).unwrap();
    let mut _conn = pool.get_conn().unwrap();

    let res = _conn.query_map(
        "select product_id, product_code, price, name from PRODUCT",
        |(product_id, product_code, price, name)| 
          Product {
                id: product_id,
                code: product_code,
                price: price,
                product_name: name
            }
    ).expect("Query failed.");
     
    web::Json(res)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello2)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}