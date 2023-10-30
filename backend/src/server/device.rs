use uuid::Uuid;

use crate::registration::registration_service::Capability;

pub struct Device {
    pub uuid: Uuid,
    pub stringified_uuid: String,
    pub capabilities: Vec<Capability>,
}
impl Device {
    pub fn new(capabilities: Vec<Capability>, uuid: Uuid) -> self::Device {
        return Device { uuid, stringified_uuid: uuid.to_string(), capabilities };
    }
}
