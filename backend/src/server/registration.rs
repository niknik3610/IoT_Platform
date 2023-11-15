use fxhash::FxHashMap;
use rsa::RsaPublicKey;
use std::sync::Arc;
use tonic::async_trait;
use uuid::Uuid;

use crate::certificate_signing::CertificateSigningService;
use crate::types::types;
use crate::{
    certificate_signing, device, frontend_clients, ConnectedDevicesType, RPCFunctionResult,
    ThreadSafeMutable,
};

use self::frontend_registration_service::frontend_registration_service_server::FrontendRegistrationService;
use self::registration_service::registration_service_server::RegistrationService;

pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

fn generate_new_id() -> Uuid {
    Uuid::new_v4()
}

pub struct ClientRegistrationHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    connected_device_uuids: ThreadSafeMutable<Vec<String>>,
    signing_service: certificate_signing::CertificateSigningService,
    _public_key: rsa::RsaPublicKey,
    string_public_key: String,
    _device_count: ThreadSafeMutable<u128>,
}
impl ClientRegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        device_count: ThreadSafeMutable<u128>,
        signing_service: CertificateSigningService,
        public_key: RsaPublicKey,
        connected_device_uuids: ThreadSafeMutable<Vec<String>>,
    ) -> ClientRegistrationHandler {
        let string_public_key = serde_json::to_string(&public_key).unwrap();
        return ClientRegistrationHandler {
            connected_devices,
            connected_device_uuids,
            _public_key: public_key,
            string_public_key,
            _device_count: device_count,
            signing_service,
        };
    }
}
#[async_trait]
impl RegistrationService for ClientRegistrationHandler {
    async fn register(
        &self,
        request: tonic::Request<self::registration_service::RegistrationRequest>,
    ) -> RPCFunctionResult<self::registration_service::RegistrationResponse> {
        let connected_devices = self.connected_devices.lock();
        let connected_devices_uuids = self.connected_device_uuids.lock();

        let request = request.into_inner();
        let client_id: Uuid = generate_new_id();

        let csr = client_id.to_string() + &request.public_key;
        let new_certificate = self.signing_service.sign_certificate(csr);

        let capabilites = request.capabilities;

        let device_public_key: rsa::RsaPublicKey =
            serde_json::from_str(&request.public_key).unwrap();

        let device = device::Device::new(capabilites, client_id, device_public_key);

        let response = registration_service::RegistrationResponse {
            public_key: self.string_public_key.clone(),
            client_id: device.stringified_uuid.clone(),
            certificate: new_certificate.await,
        };

        {
            connected_devices_uuids
                .await
                .push(device.stringified_uuid.clone());
        }

        {
            connected_devices
                .await
                .insert(device.stringified_uuid.clone(), device);
        }

        return Ok(tonic::Response::new(response));
    }
}

pub mod frontend_registration_service {
    tonic::include_proto!("frontend.registration");
}

pub type ConnectedFrontendsType = FxHashMap<String, frontend_clients::FrontendDevice>;
pub struct FrontendRegistrationHandler {
    connected_frontends: ThreadSafeMutable<ConnectedFrontendsType>,
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    connected_devices_uuids: ThreadSafeMutable<Vec<String>>,
}
impl FrontendRegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        connected_devices_uuids: ThreadSafeMutable<Vec<String>>,
    ) -> Self {
        let connected_frontends =
            Arc::new(tokio::sync::Mutex::new(ConnectedFrontendsType::default()));
        return Self {
            connected_devices,
            connected_frontends,
            connected_devices_uuids,
        };
    }
}
#[async_trait]
impl FrontendRegistrationService for FrontendRegistrationHandler {
    async fn register(
        &self,
        request: tonic::Request<self::frontend_registration_service::RegistrationRequest>,
    ) -> RPCFunctionResult<self::frontend_registration_service::RegistrationResponse> {
        let connected_frontends = self.connected_frontends.lock();
        let id = generate_new_id().to_string();
        let request = request.into_inner();

        let new_device = frontend_clients::FrontendDevice::new(id.clone(), request.device_name);
        connected_frontends.await.insert(id.clone(), new_device);

        return Ok(tonic::Response::new(
            self::frontend_registration_service::RegistrationResponse {
                client_id: id.to_string(),
            },
        ));
    }
    async fn get_connected_devices(
        &self,
        request: tonic::Request<self::frontend_registration_service::ConnectedDevicesRequest>,
    ) -> RPCFunctionResult<self::frontend_registration_service::ConnectedDevicesResponse> {
        use self::frontend_registration_service::Device;
        let mut connected_devices_uuids_clone = Vec::new();

        //todo: add caching for this list
        let mut to_return = Vec::<Device>::new();

        async {
            let connected_devices_uuids = self.connected_devices_uuids.lock().await;
            connected_devices_uuids_clone = connected_devices_uuids.clone();
        }
        .await;

        async {
            let connected_devices = self.connected_devices.lock().await;
            connected_devices_uuids_clone.iter().for_each(|device_id| {
                let device = connected_devices.get(device_id);
                if let Some(device) = device {
                    let capabilities_i32 = device
                        .capabilities
                        .iter()
                        .map(|capability| capability.capability as i32)
                        .collect();
                    let new_device = Device {
                        device_name: String::from("todo"),
                        device_uuid: device.stringified_uuid.clone(),
                        capabilities: capabilities_i32,
                    };
                    to_return.push(new_device);
                }
            });
        }
        .await;

        return Ok(tonic::Response::new(
            self::frontend_registration_service::ConnectedDevicesResponse { devices: to_return },
        ));
    }
}
