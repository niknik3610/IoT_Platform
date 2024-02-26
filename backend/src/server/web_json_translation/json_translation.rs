

use anyhow::anyhow;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::ThreadSafeMutable;

use super::json_device_control::frontend_device_control::frontend_device_control_service_client::FrontendDeviceControlServiceClient;
use super::json_registration::json_registration_service::frontend_registration_service_client::FrontendRegistrationServiceClient;

#[derive(Clone)]
pub struct TranslationClientState {
    pub registration_client: ThreadSafeMutable<FrontendRegistrationServiceClient<Channel>>,
    pub device_control_client: ThreadSafeMutable<FrontendDeviceControlServiceClient<Channel>>,
}
impl TranslationClientState {
    pub async fn new(grpc_server_addr: String) -> anyhow::Result<Self> {
        let registration_service_client =
            FrontendRegistrationServiceClient::connect(grpc_server_addr.to_string()).await;
        let registration_service_client = match registration_service_client {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                return Err(anyhow!("JSON Client was unable to connect to the server"));
            }
        };

        let device_control_service_client =
            FrontendDeviceControlServiceClient::connect(grpc_server_addr).await;
        let device_control_service_client = match device_control_service_client {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                return Err(anyhow!("JSON Client was unable to connect to the server"));
            }
        };

        return Ok(Self {
            registration_client: ThreadSafeMutable::new(Mutex::new(registration_service_client)),
            device_control_client: ThreadSafeMutable::new(Mutex::new(
                device_control_service_client,
            )),
        });
    }
}
