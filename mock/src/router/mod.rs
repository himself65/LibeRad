use actix_web::{post, get, web, Responder, HttpResponse, Error};
use serde::Deserialize;

use crate::schema::Message;

#[post("/message")]
pub async fn add_message(
    redis_client: web::Data<redis::Client>,
    message: web::Json<Message>,
) -> impl Responder {
    let id = message.id.to_string();
    let json_string = serde_json::to_string(&message.into_inner()).unwrap();
    debug!("{}", json_string.clone());
    let connection = &mut redis_client.get_connection().unwrap();
    let _: () = redis::cmd("SET")
        .arg(id)
        .arg(json_string)
        .query(connection)
        .unwrap();

    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct MessageQuery {
    id: i32
}

#[get("/message")]
pub async fn get_message(
    redis_client: web::Data<redis::Client>,
    query: web::Query<MessageQuery>,
) -> impl Responder {
    let connection = &mut redis_client.get_connection().unwrap();
    let (json): (String) = redis::cmd("GET")
        .arg(query.id.to_string())
        .query(connection)
        .unwrap();
    debug!("{}", json);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}