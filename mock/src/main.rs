use actix_web::{get, web, App, middleware, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Gender {
    Man = 0,
    Woman = 1,
}

#[derive(Serialize, Deserialize)]
struct Link {
    name: String,
    to: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
    links: Vec<Link>,
}

#[get("/user")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(User {
        name: "himself65".to_string(),
        age: 18,
        gender: Gender::Woman,
        links: vec![
            Link {
                name: "GitHub".to_string(),
                to: "https://github.com/himself65".to_string(),
            },
            Link {
                name: "Twitter".to_string(),
                to: "https://twitter.com/himself_65".to_string(),
            }
        ],
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
    )
        .bind("127.0.0.1:3002")?
        .run()
        .await
}
