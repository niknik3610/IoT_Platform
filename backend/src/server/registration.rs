use tonic::async_trait;

use crate::{RPCFunctionResult, ThreadSafeMutable};

use self::registration_service::registration_service_server::RegistrationService;
use self::registration_service::{RegistrationResponse, RegistrationRequest};

pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

pub struct RegistrationHandler {
    //TODO: add hashing scheme to allow for safer registration
    connected_devices: ThreadSafeMutable<u32>
}
impl RegistrationHandler {
    pub fn new(connected_devices: ThreadSafeMutable<u32>) -> RegistrationHandler {
        return RegistrationHandler {
            connected_devices
        }
    }
}
#[async_trait]
impl RegistrationService for RegistrationHandler {
    async fn register(&self, _request: tonic::Request<RegistrationRequest>) -> RPCFunctionResult<RegistrationResponse> {
        let mut connected_devices = self.connected_devices.lock().await;
        let client_id = *connected_devices;  
        let response = RegistrationResponse {
            client_id: client_id.to_string(),
        };

        *connected_devices += 1;
        return Ok(tonic::Response::new(response));
    }
}
