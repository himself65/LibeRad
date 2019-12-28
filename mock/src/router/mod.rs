use std::sync::Arc;

use actix_web::{post, get, web, Responder, HttpResponse, Error};
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;

use super::schema::*;
use std::ops::Deref;

#[post("/user")]
pub async fn user(
    st: web::Data<Arc<UserSchema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[get("/")]
pub async fn index() -> impl Responder {
    let html = graphiql_source("http://127.0.0.1:3002/user");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}