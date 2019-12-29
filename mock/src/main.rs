use actix_web::{web, App, middleware, HttpServer};
use crate::db::connect_redis;
use crate::router::add_message;

#[macro_use]
extern crate log;
#[macro_use]
extern crate juniper;
extern crate redis;

mod db;
mod router;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("RUST_LOG: {}", std::env::var("RUST_LOG").unwrap());
    env_logger::init();

    HttpServer::new(|| {
        let redis_connection = connect_redis();
        App::new()
            .data(redis_connection)
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(add_message)
    })
        .bind("127.0.0.1:3002")?
        .run()
        .await
}
