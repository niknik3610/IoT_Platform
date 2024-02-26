use rsa::sha2::Sha256;
use tonic::transport::Channel;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::pss::{BlindedSigningKey, Signature};

use self::polling::{PollingOption, Update};
use crate::client_types::types::DeviceCapabilityStatus;
use crate::{
    client::ThreadSafeMutable, client_connection::ServerConnection, client_types::types
};

pub mod polling {
    tonic::include_proto!("iot.polling");
}
pub struct PollingService {
    client: polling::request_update_service_client::RequestUpdateServiceClient<Channel>,
    capabilities: ThreadSafeMutable<Vec<types::DeviceCapabilityStatus>>,
    updated: ThreadSafeMutable<bool>,
    connection_details: ServerConnection,
    signing_key: BlindedSigningKey<Sha256>,
}
impl PollingService {
    pub fn new(
        client: polling::request_update_service_client::RequestUpdateServiceClient<Channel>,
        capabilities: ThreadSafeMutable<Vec<types::DeviceCapabilityStatus>>,
        updated: ThreadSafeMutable<bool>,
        connection_details: ServerConnection,
        signing_key: BlindedSigningKey<Sha256>
    ) -> Self {
        Self {
            client,
            capabilities,
            updated,
            connection_details,
            signing_key
        }
    }
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

        let timestamp = get_timestamp();
        let signature = create_signature(&self.signing_key, &certificate, &updated_capabilities, &timestamp);

        let updates = self
            .client
            .poll_for_update(tonic::Request::new(polling::PollRequest {
                certificate,
                uuid,
                updated_capabilities,
                signature,
                timestamp
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

fn get_timestamp() -> u64 {
    use std::time::SystemTime;
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}

fn create_signature(signing_key: &BlindedSigningKey<Sha256>, certificate: &String, updated_capabilities: &Vec<DeviceCapabilityStatus>, timestamp: &u64) -> Vec<u8> {
    let capability_string = updated_capabilities
        .iter()
        .map(|capability| capability.capability.clone())
        .reduce(|acc, capability| {
            acc + &capability
        })
        .unwrap_or(String::from(""));


    let to_sign = timestamp.to_string() + &capability_string + certificate;
    let mut rng = rand::thread_rng();
    signing_key.sign_with_rng(&mut rng, to_sign.as_bytes()).to_vec()
}
