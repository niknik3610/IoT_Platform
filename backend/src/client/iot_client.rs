use std::sync::Arc;

use fxhash::FxHashMap;

use crate::{
    client_config::ParsedConfig, client_polling, client_registration,
    client_types::types::DeviceCapabilityStatus,
};
pub const SERVER_IP: &str = "http://[::1]:50051";

const RSA_KEY_SIZE: usize = 2048;
const POLLING_INTERVAL: u64 = 500;
pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;

pub struct ClientHandler {
    callbacks: ThreadSafeMutable<FxHashMap<String, Box<dyn Fn() -> ()>>>,
}

impl ClientHandler {
    pub fn new() -> Self {
        Self {
            callbacks: ThreadSafeMutable::default(),
        }
    }
    ///Replaces current callback function with supplied one
    pub async fn add_callback(&mut self, capabillity: &str, call_back: Box<dyn Fn() -> ()>) {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.insert(String::from(capabillity), call_back);
    }
    pub async fn run(&mut self, config: ParsedConfig) -> anyhow::Result<()> {
        let private_key;
        let capabilities = config.capabilities;

        let capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>> =
            Arc::new(tokio::sync::Mutex::new(capabilities));
        {
            let mut rng = rand::thread_rng();
            private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
        }

        let client_connection_details = client_registration::repeated_register_self(
            &private_key.to_public_key(),
            capabilities.clone(),
            config.device_name,
        )
        .await;

        {
            let certificate = client_connection_details.security_certificate.clone();
            let uuid = client_connection_details.uuid.clone();
            let has_update = ThreadSafeMutable::new(tokio::sync::Mutex::new(false));

            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(POLLING_INTERVAL));

            let mut polling_service = client_polling::PollingService {
                client: client_polling::polling::request_update_service_client::RequestUpdateServiceClient::connect(SERVER_IP).await.unwrap(),
                capabilities: capabilities.clone(),
                updated: has_update.clone(),
            };

            loop {
                interval.tick().await;
                let updates = polling_service
                    .get_updates(certificate.clone(), uuid.clone())
                    .await;
                let updates = match updates {
                    Some(v) => v,
                    None => {
                        // println!("No updates available");
                        continue;
                    }
                };
                for update in updates.into_iter() {
                    let callbacks = self.callbacks.lock().await;
                    let callback = callbacks.get(&update.capability);
                    match callback {
                        Some(v) => v(),
                        None => println!("Received signal to {}", update.capability),
                    }
                }
            }
        }
    }
}
