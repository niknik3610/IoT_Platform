use std::time::Duration;

use client_config::ClientConfig;
use iot_client::ClientHandler;
use zeroconf::{browser::TMdnsBrowser, event_loop::TEventLoop};

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

    let _forever = tokio::task::spawn( async {    
        zero_conf_discover_services().await.unwrap();
    });

    client_handler.run(config).await.unwrap();
    return Ok(());
}

async fn zero_conf_discover_services() -> anyhow::Result<()>{
    let service_type = zeroconf::ServiceType::new("IOT_HUB_SERVICE", "tcp")?;
    let mut browser = zeroconf::MdnsBrowser::new(service_type);
    browser.set_service_discovered_callback(Box::new(|result, _context| {
        let service = result.unwrap();
        println!("Discovered service on network: {:?}", service);
    }));

    let event_loop = browser.browse_services()?;
    loop {
        event_loop.poll(Duration::from_secs(0))?;
    }
}
