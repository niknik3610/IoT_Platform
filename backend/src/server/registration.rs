use crate::{device, ConnectedDevicesType, RPCFunctionResult, ThreadSafeMutable};
use tonic::async_trait;
use uuid::Uuid;

use self::registration_service::registration_service_server::RegistrationService;
use self::registration_service::{RegistrationRequest, RegistrationResponse};

const RSA_KEY_SIZE: usize = 2048;

pub mod registration_service {
    tonic::include_proto!("iot.registration");
}

pub struct RegistrationHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    _device_count: ThreadSafeMutable<u128>,
}
impl RegistrationHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        device_count: ThreadSafeMutable<u128>,
    ) -> RegistrationHandler {
        return RegistrationHandler {
            connected_devices,
            _device_count: device_count,
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

        let private_key;
        {
            let mut rng = rand::thread_rng();
            private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap(); //TODO
        }

        let stringified_public_key = serde_json::to_string(&private_key.to_public_key()).unwrap();
        
        let capabilites = request.capabilities;

        let device_public_key: rsa::RsaPublicKey = serde_json::from_str(&request.public_key).unwrap(); 

        let csr = client_id.to_string() + &request.public_key;
        let signing_key = rsa::pkcs1v15::SigningKey::<rsa::sha2::Sha256>::new(private_key.clone()); //TODO: move out of here
                                                                                                    //
        let new_certificate = generate_new_certificate(&signing_key, csr).await;

        let device = device::Device::new(capabilites, client_id, private_key, device_public_key);

        let response = RegistrationResponse {
            public_key: stringified_public_key,
            client_id: device.stringified_uuid.clone(),
            certificate: new_certificate,
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
async fn generate_new_certificate(server_signing_key: &rsa::pkcs1v15::SigningKey<rsa::sha2::Sha256>, csr: String) -> String {
    use rsa::signature::Signer;

    let signed_rsa: String = server_signing_key.sign(&csr[..].as_bytes()).to_string();
    return signed_rsa + &csr;
}
