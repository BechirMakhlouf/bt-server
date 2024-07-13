use redis::AsyncCommands;

pub struct SessionRepository {
    redis_client: redis::Client,
}

impl SessionRepository {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }
}

#[cfg(test)]
mod tests {
    use super::SessionRepository;
    // #[tokio::test]
    // pub async fn test_redis_connection() {
    //     // let session_repo = SessionRepository::new(redis_client)
    //     session_repo.connect_redis().await.expect("why didn't work");
    // }
}
