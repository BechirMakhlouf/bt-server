use actix_multipart::form::{tempfile::TempFile, text, MultipartForm};
use actix_session::Session;
use actix_web::{error, web, Error, HttpResponse, Responder};

use crate::{
    file_storage::MediaStorage, middleware::is_authenticated, models::user_profile::UserProfile,
    services::AppState,
};

#[derive(Debug, MultipartForm)]
#[multipart(deny_unknown_fields, duplicate_field = "deny")]
pub struct FormBody {
    #[multipart(limit = "8MB")]
    picture: TempFile,
    description: text::Text<String>,
}

pub async fn post(
    body: MultipartForm<FormBody>,
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder, Error> {
    let user_id = is_authenticated(&session, &app_state)?;

    let body = body.into_inner();
    let media_storage = &app_state.media_storage;
    let user_profile_repo = &app_state.repositories.user_profile;

    let file_name = body.picture.file_name.map_or("".to_string(), |v| v);
    let media_id = format!("{}.{}", uuid::Uuid::new_v4(), file_name);

    let content_type = match body.picture.content_type {
        Some(content_type) => content_type.to_string(),
        None => return Err(error::ErrorBadRequest("content_type missing from file.")),
    };

    //TODO: convert and compress images
    media_storage
        .put(
            body.picture.file.path(),
            &media_id,
            content_type.as_str(),
            true,
        )
        .await
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))?;

    let user_profile = UserProfile::new(
        user_id.clone(),
        user_id.to_string(),
        media_id,
        body.description.to_string(),
    );

    match user_profile_repo.add(user_profile).await {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(sqlx::Error::Database(err)) => {
            if err.is_unique_violation() {
                Err(actix_web::error::ErrorConflict(err))
            } else {
                Err(actix_web::error::ErrorInternalServerError(err))
            }
        }
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
