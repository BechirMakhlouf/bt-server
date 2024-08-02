use actix_session::Session;
use actix_web::{
    cookie::{self, time::Duration, Cookie},
    get, post, web, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use tracing::span::Id;

use crate::{
    models::user::UserCredentials,
    services::{AppState, ResponseError},
    ACCESS_TOKEN_NAME,
};

pub async fn sign_out(req: HttpRequest) -> impl Responder {
    let mut cookie = Cookie::new(ACCESS_TOKEN_NAME, "");
    cookie.set_expires(cookie::time::OffsetDateTime::now_utc());

    HttpResponse::Ok().cookie(cookie).finish()
}
