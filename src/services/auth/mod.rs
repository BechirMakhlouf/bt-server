mod is_authenticated;
mod sign_in;
mod sign_out;
mod sign_up;

use actix_web::{web, HttpResponse};

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route("/sign-up", web::post().to(sign_up::sign_up))
        .route("/sign-in", web::post().to(sign_in::sign_in))
        .route(
            "/is-authenticated",
            web::get().to(is_authenticated::is_authenticated),
        )
        .route(
            "/reset-password",
            web::post().to(HttpResponse::NotImplemented),
        )
        .route(
            "/refresh-token",
            web::post().to(HttpResponse::NotImplemented),
        )
        .route("/sign-out", web::get().to(sign_out::sign_out))
}
