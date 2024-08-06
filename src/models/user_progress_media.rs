use serde::{Deserialize, Serialize};

use crate::types::past_naive_date::PastNaiveDate;

use super::user;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProgress {
    user_id: user::Id,
    description: String,
    date_at: PastNaiveDate,
    media_urls: Vec<url::Url>,
}
