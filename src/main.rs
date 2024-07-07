use tonic::transport::Server;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .add_service(services::create_reflection_service())
        .add_service(services::create_health_service())
        .add_service(services::create_test_service())
        .serve(addr)
        .await?;

    Ok(())
}
