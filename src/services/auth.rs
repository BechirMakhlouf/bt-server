#![allow(unused)]
pub mod sign_in;
pub mod sign_out;
pub mod sign_up;

use actix_web::{web, HttpResponse};

use crate::guards::cookie_guard;

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route("/sign-up", web::post().to(sign_up::sign_up))
        .route("/sign-in", web::post().to(sign_in::sign_in))
        .route(
            "/reset-password",
            web::post().to(HttpResponse::NotImplemented),
        )
        .route(
            "/is-authenticated",
            web::get().to(HttpResponse::NotImplemented),
        )
        .route(
            "/refresh-token",
            web::post().to(HttpResponse::NotImplemented),
        )
        .route("/sign-out", web::get().to(sign_out::sign_out))
}
