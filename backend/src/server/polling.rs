use self::polling_service::PollingOption;
use self::polling_service::{
    request_update_service_server::RequestUpdateService, PollRequest, PollResponse,
};
use crate::types::types::{self, Capability};
use crate::{ConnectedDevicesType, RPCFunctionResult, ThreadSafeMutable};
use fxhash::FxHashMap;
use tonic::async_trait;

pub mod polling_service {
    tonic::include_proto!("iot.polling");
}

#[derive(Clone)]
pub struct DeviceEvent {
    capability: Capability,
    _requester_uuid: String,
    value: Option<i32>,
}
impl DeviceEvent {
    pub fn new(capability: Capability, requester_uuid: String, value: Option<i32>) -> Self {
        return DeviceEvent {
            capability,
            _requester_uuid: requester_uuid,
            value,
        };
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

        return polling_service::Update {
            capability: self.capability as i32,
            has_value,
            value,
        };
    }
}

pub struct PollingHandler {
    connected_devices: ThreadSafeMutable<ConnectedDevicesType>,
    events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>,
}
impl PollingHandler {
    pub fn new(connected_devices: ThreadSafeMutable<ConnectedDevicesType>, events: ThreadSafeMutable<FxHashMap<String, Vec<DeviceEvent>>>) -> Self {
        return PollingHandler {
            connected_devices,
            events,
        };
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

        match device {
            Some(r) => r,
            None => {
                return Ok(tonic::Response::new(PollResponse {
                    has_update: PollingOption::DeviceNotFound as i32,
                    updates: Vec::new(),
                }));
            }
        };

        let mut updates = self.events.lock().await;
        let updates = updates.get_mut(&device_uuid);

        let updates = match updates {
            Some(r) => r,
            None => {
                return Ok(tonic::Response::new(PollResponse {
                    has_update: PollingOption::None as i32,
                    updates: Vec::new(),
                }));
            }
        };

        if updates.is_empty() {
            return Ok(tonic::Response::new(PollResponse {
                has_update: PollingOption::None as i32,
                updates: Vec::new(),
            }));
        };

        let updates_clone = updates
            .clone()
            .iter()
            .map(|update| update.to_update())
            .collect();
        updates.clear();

        return Ok(tonic::Response::new(PollResponse {
            has_update: PollingOption::Some as i32,
            updates: updates_clone,
        }));
    }
}
