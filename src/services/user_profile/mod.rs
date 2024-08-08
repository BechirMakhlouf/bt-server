use actix_web::{web, HttpResponse};

// mod delete;
// mod get;
mod post;
// mod put;

// use delete::delete;
// use get::get;
// use post::post;
// use put::put;

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route(
            "/{profile_url}",
            web::get().to(HttpResponse::NotImplemented),
        )
        .route("", web::get().to(HttpResponse::NotImplemented))
        .route("", web::post().to(post::post))
        .route("", web::put().to(HttpResponse::NotImplemented))
        .route("", web::delete().to(HttpResponse::NotImplemented))
}
