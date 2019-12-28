use actix_web::{web, App, middleware, HttpServer};
use crate::schema::User;

#[macro_use]
extern crate juniper;

mod schema;
mod router;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let client = mongodb::Client::with_uri_str("mongodb://root:123456@localhost:27017")
        .expect("Failed to connect mongodb");
    let db = client.database("liberad");

    let schema = std::sync::Arc::new(schema::create_user_schema());

    HttpServer::new(move ||
        App::new()
            .data(schema.clone())
            .data(std::sync::Arc::new(db.clone()))
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(router::user)
            .service(router::index)
    )
        .bind("127.0.0.1:3002")?
        .run()
        .await
}
