use anyhow::anyhow;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::ThreadSafeMutable;

use super::json_registration::json_registration_service::frontend_registration_service_client::FrontendRegistrationServiceClient;
pub const SERVER_IP: &str = "http://[::1]:50051";

#[derive(Clone)]
pub struct TranslationClientState {
    pub registration_client: ThreadSafeMutable<FrontendRegistrationServiceClient<Channel>>,
}
impl TranslationClientState {
    pub async fn new() -> anyhow::Result<Self> {
        let registration_service_client =
            FrontendRegistrationServiceClient::connect(SERVER_IP).await;
        let registration_service_client = match registration_service_client {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                return Err(anyhow!("JSON Client was unable to connect to the server"));
            }
        };

        return Ok(Self {
            registration_client: ThreadSafeMutable::new(Mutex::new(registration_service_client)),
        });
    }
}
