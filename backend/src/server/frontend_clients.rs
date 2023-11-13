pub struct FrontendDevice {
    id: String,
    device_name: String,
}
impl FrontendDevice {
    pub fn new(id: String, device_name: String) -> Self {
        return FrontendDevice { id, device_name }
    }
}
