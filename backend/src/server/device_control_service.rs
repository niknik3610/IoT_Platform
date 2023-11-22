use crate::{
    polling::DeviceEvent,
    types::types::{self, Capability},
    ThreadSafeMutable,
};
use fxhash::FxHashMap;
use tonic::async_trait;

pub mod frontend_device_control {
    tonic::include_proto!("frontend.devicecontrol");
}

pub mod device_control {
    tonic::include_proto!("iot.devicecontrol");
}

#[derive(Clone)]
pub struct FrontendDeviceControlHandler {
    events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>,
}
impl FrontendDeviceControlHandler {
    pub fn new(events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>) -> Self{
        FrontendDeviceControlHandler {
            events
        }
    }
}
#[async_trait]
impl frontend_device_control::frontend_device_control_service_server::FrontendDeviceControlService
    for FrontendDeviceControlHandler
{
    async fn control_device(
        &self,
        request: tonic::Request<frontend_device_control::DeviceControlRequest>,
    ) -> tonic::Result<tonic::Response<frontend_device_control::DeviceControlResponse>> {
        let events = self.events.lock();
        let request = request.into_inner();
        let uuid = request.device_uuid;
        let capability_to_be_triggered = request.capability;

        let new_event = DeviceEvent::new(
            Capability::try_from(capability_to_be_triggered).unwrap(),
            uuid.clone(),
            None,
        ); //todo
        {
            let mut events = events.await;
            let mut device_events = events.get_mut(&uuid);
            if let None = device_events {
                events.insert(uuid.clone(), Vec::new());
                device_events = events.get_mut(&uuid);
            }
            device_events.unwrap().push(new_event);
        }

        return Ok(tonic::Response::new(
            frontend_device_control::DeviceControlResponse { result: 1 },
        ));
    }
}

// struct DeviceControlService {
// }
// #[async_trait]
// impl device_control::device_control_service_server::DeviceControlService for DeviceControlService {
//     async fn control_device(
//         &self,
//         request: tonic::Request<device_control::DeviceControlRequest>,
//     ) -> tonic::Result<tonic::Response<device_control::DeviceControlResponse>> {
//
//     }
// }
