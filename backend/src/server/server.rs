use std::sync::Arc;

use fxhash::FxHashMap;
use polling::polling_service::{self, request_update_service_server};
use registration::registration_service::registration_service_server;
use tonic::transport::Server;

pub mod certificate_signing;
pub mod device;
pub mod polling;
mod registration;
pub mod types;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;
pub type RPCFunctionResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type ConnectedDevicesType = FxHashMap<String, device::Device>;

const RSA_KEY_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let connected_devices: ThreadSafeMutable<ConnectedDevicesType> =
        Arc::new(tokio::sync::Mutex::new(FxHashMap::default()));

    let device_count = Arc::new(tokio::sync::Mutex::new(0));

    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
    }
    let signing_key = rsa::pkcs1v15::SigningKey::<rsa::sha2::Sha256>::new(private_key.clone());

    let signing_service = certificate_signing::CertificateSigningService::new(signing_key);
    let registration_service = registration::RegistrationHandler::new(
        connected_devices.clone(),
        device_count,
        signing_service,
        private_key.to_public_key(),
    );

    let polling_service = polling::PollingHandler::new(connected_devices.clone());

    Server::builder()
        .add_service(registration_service_server::RegistrationServiceServer::new(
            registration_service,
        ))
        .add_service(request_update_service_server::RequestUpdateServiceServer::new(
            polling_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
