use actix_multipart::form::{tempfile::TempFile, text, MultipartForm};
use actix_web::{web, HttpResponse, Responder};

use crate::{file_storage::MediaStorage, services::AppState};

#[derive(Debug, MultipartForm)]
#[multipart(deny_unknown_fields, duplicate_field = "deny")]
pub struct FormBody {
    #[multipart(limit = "8MB")]
    picture: TempFile,
    description: Option<text::Text<String>>,
}

// user_id: user::Id,
// url: url::Url,
// picture_url: url::Url,
// description: String,

pub async fn post(
    body: MultipartForm<FormBody>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let body = body.into_inner();
    let media_storage = &app_state.media_storage;

    media_storage.put(&body.picture.file.path(), media_id, content_type, is_private)
    Ok(HttpResponse::Ok())
}
