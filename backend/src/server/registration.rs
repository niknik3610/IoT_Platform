use crate::certificate_signing::CertificateSigningService;
use crate::{
    certificate_signing, device, ConnectedDevicesType, RPCFunctionResult, ThreadSafeMutable,
};
use rsa::RsaPublicKey;
use tonic::async_trait;
use uuid::Uuid;

use self::registration_service::registration_service_server::RegistrationService;
use self::registration_service::{RegistrationRequest, RegistrationResponse};

pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

pub struct RegistrationHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    signing_service: certificate_signing::CertificateSigningService,
    public_key: rsa::RsaPublicKey,
    string_public_key: String,
    _device_count: ThreadSafeMutable<u128>,
}
impl RegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        device_count: ThreadSafeMutable<u128>,
        signing_service: CertificateSigningService,
        public_key: RsaPublicKey,
    ) -> RegistrationHandler {
        let string_public_key = serde_json::to_string(&public_key).unwrap();
        return RegistrationHandler {
            connected_devices,
            public_key,
            string_public_key,
            _device_count: device_count,
            signing_service,
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

        connected_devices
            .await
            .insert(device.stringified_uuid.clone(), device);

        return Ok(tonic::Response::new(response));
    }
}

fn generate_new_id() -> Uuid {
    Uuid::new_v4()
}
