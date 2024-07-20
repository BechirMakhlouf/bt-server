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
}
