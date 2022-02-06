use mysql::*;
use mysql::prelude::*;
// use chrono::prelude::*; //For date and time

struct Product {
    id: u64,
    code: String,
    price: f32,
    product_name: String,
}
fn main() {
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
     
    for r in res {
      println!("{}, {}", 
        r.id, r.product_name);
    }

    _conn.query_iter("select product_id, name from PRODUCT")
    .unwrap()
    .for_each(|row| {
        let r:(i32, String) = from_row(row.unwrap());
        println!("{}, {}", r.0, r.1);
     });   
}