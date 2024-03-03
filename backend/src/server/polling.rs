use std::sync::Arc;

use self::polling_service::PollingOption;
use self::polling_service::{
    request_update_service_server::RequestUpdateService, PollRequest, PollResponse,
};
use crate::certificate_signing::CertificateSigningService;
use crate::types::types::{self, DeviceCapabilityStatus};
use crate::{certificate_signing, ConnectedDevicesType, RPCFunctionResult, ThreadSafeMutable};
use fxhash::FxHashMap;
use tonic::async_trait;

pub mod polling_service {
    tonic::include_proto!("iot.polling");
}

#[derive(Clone)]
pub struct DeviceEvent {
    capability: String,
    _requester_uuid: String,
    value: Option<i32>,
}
impl DeviceEvent {
    pub fn new(capability: String, requester_uuid: String, value: Option<i32>) -> Self {
        DeviceEvent {
            capability,
            _requester_uuid: requester_uuid,
            value,
        }
    }
    pub fn to_update(&self) -> polling_service::Update {
        let value;
        let has_value = match self.value {
            Some(v) => {
                value = v;
                true
            }
            None => {
                value = 0;
                false
            }
        };

        polling_service::Update {
            capability: self.capability.clone(),
            has_value,
            value,
        }
    }
}

pub struct PollingHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>,
    frontend_cache_valid: ThreadSafeMutable<bool>,
    certificate_signing_service: Arc<CertificateSigningService>,
}
impl PollingHandler {
    pub fn new(
        connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
        events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>,
        frontend_cache_valid: ThreadSafeMutable<bool>,
        certificate_signing_service: Arc<CertificateSigningService>,
    ) -> Self {
        PollingHandler {
            connected_devices,
            events,
            frontend_cache_valid,
            certificate_signing_service,
        }
    }
}

#[async_trait]
impl RequestUpdateService for PollingHandler {
    async fn poll_for_update(
        &self,
        request: tonic::Request<PollRequest>,
    ) -> RPCFunctionResult<PollResponse> {
        let request = request.into_inner();
        let device_uuid = request.uuid;
        let device_certificate;
        {
            let mut connected_devices = self.connected_devices.lock().await;
            let device = connected_devices.get_mut(&device_uuid);
            let device = match device {
                Some(r) => r,
                None => {
                    return Ok(tonic::Response::new(PollResponse {
                        has_update: PollingOption::DeviceNotFound as i32,
                        updates: Vec::new(),
                        signature: vec![],
                        timestamp: certificate_signing::get_timestamp(),
                    }));
                }
            };
            let signature_valid = CertificateSigningService::verify_signature_update_request(
                &device.certificate,
                &request.updated_capabilities,
                request.timestamp,
                request.signature,
                &device.device_verification_key
            );

            if !signature_valid {
                println!("Client sent an invalid signature");
                return Ok(tonic::Response::new(PollResponse {
                    has_update: PollingOption::InvalidSignature as i32,
                    updates: Vec::new(),
                    signature: vec![],
                    timestamp: certificate_signing::get_timestamp(),
                }));
            }
            if !request.updated_capabilities.is_empty() {
                let (active_capabilities, inactive_capabilities): (
                    Vec<DeviceCapabilityStatus>,
                    Vec<DeviceCapabilityStatus>,
                ) = request
                    .updated_capabilities
                    .into_iter()
                    .partition(|capability| capability.available);
                device.replace_capabilities(active_capabilities, inactive_capabilities);
                let mut frontend_cache_valid = self.frontend_cache_valid.lock().await;
                *frontend_cache_valid = false;
            }

            device_certificate = device.certificate.clone();
        }
        {
            let mut updates = self.events.lock().await;
            let updates = updates.get_mut(&device_uuid);

            let updates = match updates {
                Some(r) => r,
                None => {
                    let signature = fields_to_signature_data(&vec![], &device_certificate);
                    let (signature, timestamp) = self.certificate_signing_service.sign_data(signature);
                    return Ok(tonic::Response::new(PollResponse {
                        has_update: PollingOption::None as i32,
                        updates: Vec::new(),
                        signature,
                        timestamp,
                    }));
                }
            };

            if updates.is_empty() {
                let signature = fields_to_signature_data(&updates, &device_certificate);
                let (signature, timestamp) = self.certificate_signing_service.sign_data(signature);

                return Ok(tonic::Response::new(PollResponse {
                    has_update: PollingOption::None as i32,
                    updates: Vec::new(),
                    signature,
                    timestamp,
                }));
            };
            let signature = fields_to_signature_data(&updates, &device_certificate);
            let (signature, timestamp) = self.certificate_signing_service.sign_data(signature);

            let updates_clone = updates
                .clone()
                .iter()
                .map(|update| update.to_update())
                .collect();
            updates.clear();

            return Ok(tonic::Response::new(PollResponse {
                has_update: PollingOption::Some as i32,
                updates: updates_clone,
                signature,
                timestamp,
            }));
        }
    }
}
fn fields_to_signature_data(updates: &Vec<DeviceEvent>, certificate: &String) -> String {
    let updates = updates
        .iter()
        .map(|update| update.capability.clone())
        .reduce(|acc, capability| acc + &capability)
        .unwrap_or(String::from(""));

    certificate.clone() + &updates
}
