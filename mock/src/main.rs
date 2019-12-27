use actix_web::{web, App, middleware, HttpServer};

#[macro_use]
extern crate juniper;

mod schema;
mod router;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(router::index)
            .service(router::user)
    )
        .bind("127.0.0.1:3002")?
        .run()
        .await
}
