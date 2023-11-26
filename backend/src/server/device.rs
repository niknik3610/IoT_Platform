use crate::registration::registration_service::DeviceCapabilityStatus;
use uuid::Uuid;

#[derive(Clone)]
pub struct Device {
    pub name: String,
    pub uuid: Uuid,
    pub stringified_uuid: String,
    pub capabilities: Vec<DeviceCapabilityStatus>,
    pub device_public_key: rsa::RsaPublicKey,
}
impl Device {
    pub fn new(
        name: String,
        capabilities: Vec<DeviceCapabilityStatus>,
        uuid: Uuid,
        device_public_key: rsa::RsaPublicKey,
    ) -> self::Device {
        return Device {
            name,
            uuid,
            stringified_uuid: uuid.to_string(),
            capabilities,
            device_public_key,
        };
    }
}
