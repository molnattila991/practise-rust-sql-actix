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

pub struct AppState {
    pub pool: mysql::Pool,
}

#[get("/")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/jsonList")]
pub async fn get_list(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().unwrap();

    let res = conn.query_map(
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
pub async fn get_list3(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().unwrap();
    
    let res = conn.query_map(
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