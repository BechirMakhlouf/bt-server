use serde::{Deserialize, Serialize};

use super::user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: user::Id,
    pub url: url::Url,
    pub picture_url: url::Url,
    pub description: String,
}

impl UserProfile {
    pub fn new(
        user_id: user::Id,
        url: url::Url,
        picture_url: url::Url,
        description: String,
    ) -> Self {
        Self {
            user_id,
            url,
            picture_url,
            description,
        }
    }
}
