pub struct FrontendDevice {
    _id: String,
    _device_name: String,
}
impl FrontendDevice {
    pub fn new(id: String, device_name: String) -> Self {
        return FrontendDevice {
            _id: id,
            _device_name: device_name,
        };
    }
}
