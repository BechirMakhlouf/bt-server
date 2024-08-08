use serde::{Deserialize, Serialize};

use super::user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: user::Id,
    pub url: String,
    pub picture_id: String,
    pub description: String,
}

impl UserProfile {
    pub fn new(user_id: user::Id, url: String, picture_id: String, description: String) -> Self {
        Self {
            user_id,
            url,
            picture_id,
            description,
        }
    }
}
