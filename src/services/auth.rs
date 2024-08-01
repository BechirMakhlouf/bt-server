#![allow(unused)]
pub mod sign_in;
pub mod sign_up;

use actix_web::{web, HttpResponse};
use sign_in::*;
use sign_up::*;

pub fn get_scope<'a>(path: &'a str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route("/sign-up", web::post().to(sign_up))
        .route(
            "/sign-in",
            web::post().to(|| HttpResponse::NotImplemented()),
        )
        .route(
            "/sign-out",
            web::post().to(|| HttpResponse::NotImplemented()),
        )
        .route(
            "/reset-password",
            web::post().to(|| HttpResponse::NotImplemented()),
        )
        .route(
            "/is-authenticated",
            web::get().to(|| HttpResponse::NotImplemented()),
        )
        .route(
            "/refresh-token",
            web::get().to(|| HttpResponse::NotImplemented()),
        )
}
