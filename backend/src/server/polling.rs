use self::polling_service::PollingOption;
use self::polling_service::{
    request_update_service_server::RequestUpdateService, PollRequest, PollResponse,
};
use crate::types::types::{self};
use crate::{ConnectedDevicesType, RPCFunctionResult, ThreadSafeMutable};
use tonic::async_trait;

pub mod polling_service {
    tonic::include_proto!("iot.polling");
}

pub struct PollingHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
}
impl PollingHandler {
    pub fn new(connected_devices: ThreadSafeMutable<ConnectedDevicesType>) -> Self {
        return PollingHandler { connected_devices };
    }
}
#[async_trait]
impl RequestUpdateService for PollingHandler {
    async fn poll_for_update(
        &self,
        request: tonic::Request<PollRequest>,
    ) -> RPCFunctionResult<PollResponse> {
        let connected_devices = self.connected_devices.lock().await;
        let request = request.into_inner();
        let device_uuid = request.uuid;
        let device = connected_devices.get(&device_uuid);

        let device = match device {
            Some(r) => r,
            None => {
                return Ok(tonic::Response::new(PollResponse {
                    update: PollingOption::DeviceNotFound as i32,
                    capability: Vec::new(),
                }))
            }
        };

        let mut updates = device.updates.lock().await;

        if updates.len() == 0 {
            return Ok(tonic::Response::new(PollResponse {
                update: PollingOption::None as i32,
                capability: Vec::new(),
            }));
        };

        let updates_clone = updates.clone();
        updates.clear();

        return Ok(tonic::Response::new(PollResponse {
            update: PollingOption::Some as i32,
            capability: updates_clone,
        }));
    }
}
