use std::sync::Arc;

use actix_web::{get, web, App, middleware, HttpServer, Responder, HttpResponse};
use juniper::http::GraphQLRequest;

use super::schema::*;

#[get("/user")]
pub async fn user(
    st: web::Data<Arc<UserSchema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[get("/user/example")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}