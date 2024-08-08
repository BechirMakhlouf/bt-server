use actix_web::web;

mod delete;
mod get;
mod post;
mod put;

use delete::delete;
use get::get;
use post::post;
use put::put;

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .route("", web::get().to(get))
        .route("", web::post().to(post))
        .route("", web::put().to(put))
        .route("", web::delete().to(delete))
}
