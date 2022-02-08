mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::product_controller::{get_list, get_list3, say_hello, AppState};
use mysql::Opts;
use mysql::Pool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://shop-api:shop-api@db:3306/shop-api";
    // let url = "mysql://shop-api:shop-api@localhost:3306/shop-api";

    let opt = Opts::from_url(url);
    // let pool = Pool::new_manual(1,10, opt.unwrap()).unwrap();
    let pool = Pool::new(opt.unwrap()).unwrap();
    let mut _conn = pool.get_conn().unwrap();

    let app_data = web::Data::new(AppState { pool: pool });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(say_hello)
            .service(get_list)
            .service(get_list3)
    })
    .bind("0.0.0.0:12080")?
    .run()
    .await
}
