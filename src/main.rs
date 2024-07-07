use tonic::transport::Server;
use tonic_health::{pb::health_server::HealthServer, server::HealthService};

// mod proto {
//     tonic::include_proto!("health");
//
//     pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
//         tonic::include_file_descriptor_set!("helloworld_descriptor");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    // let reflection_service = tonic_reflection::server::Builder::configure()
    //     .register_encoded_file_descriptor_set(health::FILE_DESCRIPTOR_SET)
    //     .build()?;

    // let service = tonic_reflection::server::Builder::configure()
    //     .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    //     .build()
    //     .unwrap();
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_not_serving::<HealthServer<HealthService>>()
        .await;

    // .set_serving::<GreeterServer<MyGreeter>>()

    Server::builder()
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}
