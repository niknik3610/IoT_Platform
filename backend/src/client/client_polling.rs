use tonic::transport::Channel;

use self::polling::{PollingOption, Update};
use crate::{
    client_types::types::{self},
    ThreadSafeMutable,
};

pub mod polling {
    tonic::include_proto!("iot.polling");
}
pub struct PollingService {
    pub client: polling::request_update_service_client::RequestUpdateServiceClient<Channel>,
    pub capabilities: ThreadSafeMutable<Vec<types::DeviceCapabilityStatus>>,
    pub updated: ThreadSafeMutable<bool>,
}
impl PollingService {
    pub async fn get_updates(&mut self, certificate: String, uuid: String) -> Option<Vec<Update>> {
        let updated_capabilities = {
            let mut update = self.updated.lock().await;

            if !*update {
                Vec::new()
            } else {
                *update = false;
                self.capabilities.lock().await.clone()
            }
        };

        let updates = self
            .client
            .poll_for_update(tonic::Request::new(polling::PollRequest {
                certificate,
                uuid,
                updated_capabilities,
            }))
            .await;

        let response = match updates {
            Ok(r) => r.into_inner(),
            Err(s) => {
                println!("There was an error while polling");
                eprint!("{}", s);
                return None;
            }
        };

        match PollingOption::try_from(response.has_update).unwrap() {
            PollingOption::None => return None,
            PollingOption::Some => return Some(response.updates),
            PollingOption::Unknown => {
                println!("Something is going on, PollingOption Unknown has been returned");
                if response.updates.is_empty() {
                    return None;
                }
                return Some(response.updates);
            }
            PollingOption::DeviceNotFound => {
                println!("Your device has not been found, try registering again");
                return None;
            }
        }
    }
}
