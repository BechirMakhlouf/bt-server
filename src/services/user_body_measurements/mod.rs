use actix_web::web::{self};

mod delete;
mod get;
mod get_all;
mod patch;
mod post;

use delete::delete;
use get::get;
use get_all::get_all;
use patch::patch;
use post::post;

pub fn get_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        //TODO: this returns a vector of all the userweights but
        //also repeatedly returns the user_id in each vector element.
        .route("/all", web::get().to(get_all))
        .route("", web::get().to(get))
        .route("", web::put().to(post))
        .route("", web::patch().to(patch))
        .route("", web::delete().to(delete))
}
