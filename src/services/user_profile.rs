use actix_web::{web, HttpResponse};

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route(
            "/{profile_url}",
            web::get().to(HttpResponse::NotImplemented),
        )
        .route("", web::get().to(HttpResponse::NotImplemented))
        .route("", web::post().to(HttpResponse::NotImplemented))
        .route("", web::put().to(HttpResponse::NotImplemented))
        .route("", web::delete().to(HttpResponse::NotImplemented))
}
