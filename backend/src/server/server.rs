use std::sync::Arc;

use tonic::transport::Server;
use registration::registration_service::registration_service_server;

mod registration;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;
pub type RPCFunctionResult<T> = Result<tonic::Response<T>, tonic::Status>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let connected_devices = Arc::new(tokio::sync::Mutex::new(0));

    let registration_service = registration::RegistrationHandler::new(connected_devices);

    Server::builder()
        .add_service(registration_service_server::RegistrationServiceServer::new(registration_service))
        .serve(addr)
        .await?;

    Ok(())
}
