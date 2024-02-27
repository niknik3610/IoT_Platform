use rsa::{pss::VerifyingKey, sha2::Sha256};

use uuid::Uuid;

use crate::types::types::DeviceCapabilityStatus;

#[derive(Clone)]
pub struct Device {
    pub name: String,
    pub uuid: Uuid,
    pub stringified_uuid: String,
    pub active_capabilities: Vec<DeviceCapabilityStatus>,
    pub inactive_capabilities: Vec<DeviceCapabilityStatus>,
    pub device_public_key: rsa::RsaPublicKey,
    pub device_verification_key: VerifyingKey<Sha256>,
    pub certificate: String,
}
impl Device {
    pub fn new(
        name: String,
        active_capabilities: Vec<DeviceCapabilityStatus>,
        inactive_capabilities: Vec<DeviceCapabilityStatus>,
        uuid: Uuid,
        device_public_key: rsa::RsaPublicKey,
        device_verification_key: VerifyingKey<Sha256>,
        certificate: String,
    ) -> self::Device {
        Device {
            name,
            uuid,
            stringified_uuid: uuid.to_string(),
            active_capabilities,
            inactive_capabilities,
            device_public_key,
            device_verification_key,
            certificate,
        }
    }
    pub fn replace_capabilities(
        &mut self,
        active_capabilities: Vec<DeviceCapabilityStatus>,
        inactive_capabilities: Vec<DeviceCapabilityStatus>,
    ) {
        self.active_capabilities = active_capabilities;
        self.inactive_capabilities = inactive_capabilities;
    }
}
