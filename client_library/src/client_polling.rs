use rsa::{pkcs1v15::{Signature, SigningKey}, sha2::{digest::{FixedOutput, FixedOutputReset, HashMarker}, Sha256}, Pkcs1v15Sign, RsaPrivateKey};
use tonic::transport::Channel;
use rsa::signature::{Keypair,RandomizedSigner, SignatureEncoding, Verifier};
use rsa::pss::BlindedSigningKey;

use self::polling::{PollingOption, Update};
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
        let signature = {
            let mut rng = rand::thread_rng();
            let data = b"hello world";
            self.signing_key.sign_with_rng(&mut rng, data).to_string()
        };

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
                signature,
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
