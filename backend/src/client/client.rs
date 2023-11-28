use std::sync::Arc;

use client_types::types::{Capability, DeviceCapabilityStatus};
use futures::future::join_all;
use tokio::task;

pub mod client_connection;
pub mod client_polling;
pub mod client_registration;
pub mod client_types;

pub const SERVER_IP: &str = "http://[::1]:50051";
const RSA_KEY_SIZE: usize = 2048;
const POLLING_INTERVAL: u64 = 500;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;

//todo: fix disconnect error, probably in polling
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key;
    let capabilities = vec![
        DeviceCapabilityStatus {
            available: false,
            capability: Capability::TurnOff as i32,
        },
        DeviceCapabilityStatus {
            available: true,
            capability: Capability::TurnOn as i32,
        },
    ];

    let capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>> =
        Arc::new(tokio::sync::Mutex::new(capabilities));
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
    }

    let client_connection_details = client_registration::repeated_register_self(
        &private_key.to_public_key(),
        capabilities.clone(),
    )
    .await;

    let forever;
    {
        let certificate = client_connection_details.security_certificate.clone();
        let uuid = client_connection_details.uuid.clone();
        let has_update = ThreadSafeMutable::new(tokio::sync::Mutex::new(false));

        forever = task::spawn(async move {
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
                let futures = updates.iter().map(|update| async {
                    let capability = Capability::try_from(update.capability).unwrap();
                    //todo: find a way for this to run user defined functions (defined in a different file)
                    //todo: match these to user defined strings, not enums
                    match capability {
                        Capability::None => {}
                        Capability::TurnOn => {
                            println!("Received Request to Turn On");
                            async {
                                let mut capabilities = capabilities.lock().await;
                                capabilities.get_mut(0).unwrap().available = true;
                                capabilities.get_mut(1).unwrap().available = false;
                                *polling_service.updated.lock().await = true;
                            }
                            .await;
                        }
                        Capability::TurnOff => {
                            async {
                                println!("Received Request to Turn Off");
                                let mut capabilities = capabilities.lock().await;
                                capabilities.get_mut(1).unwrap().available = false;
                                capabilities.get_mut(0).unwrap().available = true;
                                *polling_service.updated.lock().await = true;
                            }
                            .await;
                        }
                        Capability::Unknown => {
                            println!("Unknown Request Received");
                        }
                    };
                });
                join_all(futures).await;
            }
        });
    }

    forever.await.unwrap();

    Ok(())
}
