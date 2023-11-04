use std::sync::Arc;

use fxhash::FxHashMap;
use registration::registration_service::registration_service_server;
use tonic::transport::Server;

pub mod device;
mod registration;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;
pub type RPCFunctionResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type ConnectedDevicesType = FxHashMap<String, device::Device>;

//TODO add either documentation of required OpenSSL version or make logic to check for it
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let connected_devices: ThreadSafeMutable<ConnectedDevicesType> =
        Arc::new(tokio::sync::Mutex::new(FxHashMap::default()));

    let device_count = Arc::new(tokio::sync::Mutex::new(0));

    let registration_service =
        registration::RegistrationHandler::new(connected_devices, device_count);

    Server::builder()
        .add_service(registration_service_server::RegistrationServiceServer::new(
            registration_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
