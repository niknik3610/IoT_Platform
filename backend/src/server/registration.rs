use fxhash::FxHashMap;
use rsa::pss::VerifyingKey;
use rsa::sha2::Sha256;
use rsa::RsaPublicKey;
use std::borrow::Borrow;
use std::sync::Arc;
use tonic::async_trait;
use uuid::Uuid;

use crate::certificate_signing::CertificateSigningService;
use crate::{
    certificate_signing, device, frontend_clients, ConnectedDevicesType, RPCFunctionResult,
    ThreadSafeMutable,
};

use self::frontend_registration_service::frontend_registration_service_server::FrontendRegistrationService;
use self::registration_service::registration_service_server::RegistrationService;

use crate::types::types;
pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

fn generate_new_id() -> Uuid {
    Uuid::new_v4()
}

pub struct ClientRegistrationHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    connected_device_uuids: ThreadSafeMutable<Vec<String>>,
    signing_service: ThreadSafeMutable<certificate_signing::CertificateSigningService>,
    _public_key: rsa::RsaPublicKey,
    string_public_key: String,
    _device_count: ThreadSafeMutable<u128>,
    frontend_cache_valid: ThreadSafeMutable<bool>,
}
impl ClientRegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        device_count: ThreadSafeMutable<u128>,
        signing_service: ThreadSafeMutable<CertificateSigningService>,
        public_key: RsaPublicKey,
        connected_device_uuids: ThreadSafeMutable<Vec<String>>,
        frontend_cache_valid: ThreadSafeMutable<bool>,
    ) -> ClientRegistrationHandler {
        let string_public_key = serde_json::to_string(&public_key).unwrap();
        ClientRegistrationHandler {
            connected_devices,
            connected_device_uuids,
            _public_key: public_key,
            string_public_key,
            _device_count: device_count,
            signing_service,
            frontend_cache_valid,
        }
    }
}
#[async_trait]
impl RegistrationService for ClientRegistrationHandler {
    async fn register(
        &self,
        request: tonic::Request<self::registration_service::RegistrationRequest>,
    ) -> RPCFunctionResult<self::registration_service::RegistrationResponse> {
        let request = request.into_inner();
        let client_id: Uuid = generate_new_id();

        let csr = client_id.to_string() + &request.public_key;

        let new_certificate = {
            let signing_service = self.signing_service.lock().await;
            signing_service.gen_certificate(&csr)
        };

        let capabilites = request.capabilities;

        let device_public_key: rsa::RsaPublicKey =
            serde_json::from_str(&request.public_key).unwrap();

        let (active_capabilities, inactive_capabilities) = capabilites
            .into_iter()
            .partition(|capability| capability.available);

        let certificate = new_certificate;
        let device = device::Device::new(
            request.name,
            active_capabilities,
            inactive_capabilities,
            client_id,
            device_public_key.clone(),
            VerifyingKey::<Sha256>::from(device_public_key),
            certificate.to_owned()
        );

        let response = registration_service::RegistrationResponse {
            public_key: self.string_public_key.clone(),
            client_id: device.stringified_uuid.clone(),
            certificate: certificate.to_owned()
        };

        {
            let connected_devices_uuids = self.connected_device_uuids.lock();
            connected_devices_uuids
                .await
                .push(device.stringified_uuid.clone());
        }

        {
            let mut connected_devices = self.connected_devices.lock().await;
            connected_devices.insert(device.stringified_uuid.clone(), device);
        }
        {
            let mut frontend_cache_valid = self.frontend_cache_valid.lock().await;
            *frontend_cache_valid = false;
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
    cached_devices_for_frontends:
        ThreadSafeMutable<Vec<self::frontend_registration_service::Device>>,
    cache_valid: ThreadSafeMutable<bool>,
}
impl FrontendRegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        connected_devices_uuids: ThreadSafeMutable<Vec<String>>,
        cache_valid: ThreadSafeMutable<bool>,
    ) -> Self {
        let connected_frontends =
            Arc::new(tokio::sync::Mutex::new(ConnectedFrontendsType::default()));
        let cached_devices_for_frontends = ThreadSafeMutable::default();
        return Self {
            connected_devices,
            connected_frontends,
            connected_devices_uuids,
            cached_devices_for_frontends,
            cache_valid,
        };
    }
}
#[async_trait]
impl FrontendRegistrationService for FrontendRegistrationHandler {
    async fn register(
        &self,
        request: tonic::Request<self::frontend_registration_service::RegistrationRequest>,
    ) -> RPCFunctionResult<self::frontend_registration_service::RegistrationResponse> {
        let id = generate_new_id().to_string();
        let request = request.into_inner();

        let new_device = frontend_clients::FrontendDevice::new(id.clone(), request.device_name);
        {
            let connected_frontends = self.connected_frontends.lock();
            connected_frontends.await.insert(id.clone(), new_device);
        }
        {
            let mut cache_valid = self.cache_valid.lock().await;
            *cache_valid = false;
        }

        return Ok(tonic::Response::new(
            self::frontend_registration_service::RegistrationResponse {
                client_id: id.to_string(),
            },
        ));
    }
    //todo: move this out of here
    async fn get_connected_devices(
        &self,
        _request: tonic::Request<self::frontend_registration_service::ConnectedDevicesRequest>,
    ) -> RPCFunctionResult<self::frontend_registration_service::ConnectedDevicesResponse> {
        if *self.cache_valid.lock().await {
            return Ok(tonic::Response::new(
                self::frontend_registration_service::ConnectedDevicesResponse {
                    devices: self.cached_devices_for_frontends.lock().await.clone(),
                },
            ));
        }
        use self::frontend_registration_service::Device;
        let mut connected_devices_uuids_clone = Vec::new();

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
                    let new_device = Device {
                        device_name: device.name.clone(),
                        device_uuid: device.stringified_uuid.clone(),
                        capabilities: device.active_capabilities.clone(),
                    };
                    to_return.push(new_device);
                }
            });
        }
        .await;

        {
            *self.cached_devices_for_frontends.lock().await = to_return.clone();
        }
        {
            *self.cache_valid.lock().await = true;
        }

        return Ok(tonic::Response::new(
            self::frontend_registration_service::ConnectedDevicesResponse { devices: to_return },
        ));
    }
}
