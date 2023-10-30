use crate::{ RPCFunctionResult, ThreadSafeMutable, ConnectedDevicesType, device};
use tonic::async_trait;
use uuid::Uuid;

use self::registration_service::registration_service_server::RegistrationService;
use self::registration_service::{RegistrationRequest, RegistrationResponse};

pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

pub struct RegistrationHandler {
    //TODO: add hashing scheme to allow for safer registration
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    device_count: ThreadSafeMutable<u128>,
}
impl RegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        device_count: ThreadSafeMutable<u128>,
    ) -> RegistrationHandler {
        return RegistrationHandler {
            connected_devices,
            device_count,
        };
    }
}
#[async_trait]
impl RegistrationService for RegistrationHandler {
    async fn register(
        &self,
        request: tonic::Request<RegistrationRequest>,
    ) -> RPCFunctionResult<RegistrationResponse> {
        let connected_devices = self.connected_devices.lock();
        let client_id: Uuid;
        client_id = generate_new_id().await;
        
        let capabilites = request.into_inner().capabilities;
        let device = device::Device::new(capabilites, client_id);

        let response = RegistrationResponse {
            client_id: device.stringified_uuid.clone(),
        };

        connected_devices.await.insert(device.stringified_uuid.clone(), device);

        return Ok(tonic::Response::new(response));
    }
}

async fn generate_new_id() -> Uuid {
    Uuid::new_v4()
}
