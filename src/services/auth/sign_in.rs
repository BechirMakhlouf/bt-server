use actix_session::Session;
use actix_web::{
    cookie::{Cookie, SameSite},
    get, post, web, HttpResponse, Responder,
};
use chrono::Days;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tracing::span::Id;

use crate::{
    models::{
        session::{self, SessionFactory},
        user::{self, UserCredentials},
    },
    services::{AppState, ResponseError},
    ACCESS_TOKEN_NAME,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequestBody {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    body: web::Json<SignInRequestBody>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let SignInRequestBody { password, email } = body.into_inner();
    let app_state = app_state.into_inner();

    let user_credentials = match UserCredentials::new(&email, &password) {
        Ok(user) => user,
        Err(error) => return HttpResponse::BadRequest().json(ResponseError::new(&error)),
    };

    let user_repo = &app_state.repositories.user_repository;

    let opt_user = match user_repo.get_by_email(&user_credentials.email).await {
        Ok(opt_user) => opt_user,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ResponseError::new("internal error."))
        }
    };

    match opt_user {
        Some(user) => {
            let session_factory = &app_state.session_factory;
            let user_session = session::Session::new(user.id, 500);
            let jwt = match session_factory.create_session_jwt(user_session) {
                Ok(jwt) => jwt,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            let cookie = Cookie::new(ACCESS_TOKEN_NAME, jwt);
            HttpResponse::Ok().cookie(cookie).finish()
        }
        None => HttpResponse::Unauthorized().json(ResponseError::new("Invalid Credentials")),
    }
}
