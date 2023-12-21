use client_config::ClientConfig;
use iot_client::ClientHandler;

use crate::client_config::ParsedConfig;

pub mod client_config;
pub mod client_connection;
pub mod client_polling;
pub mod client_registration;
pub mod client_types;
pub mod iot_client;

const CONFIG_PATH: &str = "./example_config.toml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ClientConfig::load_config(CONFIG_PATH);
    let config = match config {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading config: {}", e.to_string());
            println!("Loading default config...");
            ParsedConfig::default()
        }
    };

    let mut client_handler = ClientHandler::new();
    client_handler
        .add_callback("Turn On", Box::new(|| println!("Beep boop i must turn on")))
        .await;
    client_handler
        .add_callback("Turn Off", Box::new(|| println!("I am turning off :(")))
        .await;

    client_handler.run(config).await.unwrap();
    return Ok(());
}
