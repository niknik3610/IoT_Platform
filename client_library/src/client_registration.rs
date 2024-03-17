use rsa::RsaPublicKey;
use std::time::Duration;
use tokio::time::sleep;

use self::client_registration_service::RegistrationRequest;
use crate::client::ThreadSafeMutable;
use crate::client_connection;
use crate::client_registration::client_registration_service::registration_service_client::RegistrationServiceClient;
use crate::client_types::types::{self, DeviceCapabilityStatus};

mod client_registration_service {
    tonic::include_proto!("iot.registration");
}

pub async fn register_self(
    public_key: &RsaPublicKey,
    capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>>,
    device_name: String,
    server_ip: String,
) -> anyhow::Result<client_connection::ServerConnection> {
    let mut client = RegistrationServiceClient::connect(server_ip).await?;
    let stringified_public_key: String = serde_json::to_string(public_key).unwrap();

    let response;
    {
        let capabilities = capabilities.lock().unwrap();
        response = client.register(RegistrationRequest {
            name: device_name,
            public_key: stringified_public_key,
            capabilities: capabilities.clone(),
        });
    }

    let response = response.await?.into_inner();
    let server_public_key = serde_json::from_str(&response.public_key).unwrap();

    println!("Received Certificate");
    return Ok(client_connection::ServerConnection::new(
        response.client_id,
        server_public_key,
        response.certificate,
    ));
}

///Repeats the register_self operation until success
pub async fn repeated_register_self(
    public_key: &RsaPublicKey,
    capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>>,
    device_name: String,
    server_ip: String,
) -> client_connection::ServerConnection {
    println!("Attempting to establish connection with: {}", server_ip);
    let id = loop {
        let id_req_result = register_self(
            public_key,
            capabilities.clone(),
            device_name.clone(),
            server_ip.clone(),
        )
        .await;

        match id_req_result {
            Ok(r) => {
                break r;
            }
            Err(ref e) => {
                eprintln!("Error During ID request, trying again");
                eprintln!("{e}");
                sleep(Duration::from_millis(1000)).await;
            }
        }
    };

    println!("Successfully established Connection");
    return id;
}
