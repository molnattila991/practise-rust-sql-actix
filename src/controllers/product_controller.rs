use mysql::*;
use mysql::prelude::*;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: u64,
    code: String,
    price: f32,
    product_name: String,
}

#[get("/")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/jsonList")]
pub async fn get_list() -> impl Responder {
    // let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@172.21.192.1:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@db:3306/shop-api";
    let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";

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

#[get("/list")]
pub async fn get_list2() -> impl Responder {
    // let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@172.21.192.1:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@db:3306/shop-api";
    let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";

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