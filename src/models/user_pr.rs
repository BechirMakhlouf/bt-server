use serde::{Deserialize, Serialize};

use crate::types::past_naive_date::PastNaiveDate;

use super::user;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPr {
    user_id: user::Id,
    title: String,
    description: String,
    tags: Vec<String>,
    media_urls: Vec<String>,
    date_at: PastNaiveDate,
}
