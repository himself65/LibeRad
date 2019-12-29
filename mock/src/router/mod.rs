use actix_web::{post, get, web, Responder, HttpResponse, Error};

use crate::schema::Message;

#[post("/message")]
pub async fn add_message(
    redis_connection: web::Data<redis::Connection>,
    message: web::Json<Message>,
) -> impl Responder {
    debug!("{}", serde_json::to_string(&message.into_inner()).unwrap());
    // todo: save to redis
    HttpResponse::Ok()
}
