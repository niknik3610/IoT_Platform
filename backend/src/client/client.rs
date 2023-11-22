use client_types::types::Capability;
use tokio::task;

pub mod client_connection;
pub mod client_polling;
pub mod client_registration;
pub mod client_types;

pub const SERVER_IP: &str = "http://[::1]:50051";
const RSA_KEY_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap(); //TODO
    }

    let client_connection_details =
        client_registration::repeated_register_self(&private_key.to_public_key()).await;

    let forever;
    {
        let certificate = client_connection_details.security_certificate.clone();
        let uuid = client_connection_details.uuid.clone();
        forever = task::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

            let mut polling_service = client_polling::PollingService {
                client: client_polling::polling::request_update_service_client::RequestUpdateServiceClient::connect(SERVER_IP).await.unwrap(),
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
                updates.iter().for_each(|update| {
                    let capability = Capability::try_from(update.capability).unwrap();
                    println!("Received a request to: {}", capability.as_str_name());
                });
            }
        });
    }

    forever.await.unwrap();

    Ok(())
}
