use client_config::ClientConfig;
use iot_client::ClientHandler;
use rppal::gpio::Gpio;

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
        .add_callback("Turn On", Box::new(turn_on_led))
        .await;
    client_handler
        .add_callback("Turn Off", Box::new(turn_off_led))
        .await;

    client_handler.run(config).await.unwrap();
    return Ok(());
}

const GPIO_LED_PIN: u8 = 2;
fn turn_on_led() {
    let mut pin = Gpio::new()
        .unwrap()
        .get(GPIO_LED_PIN)
        .unwrap()
        .into_output();
    pin.set_high();
}

fn turn_off_led() {
    let mut pin = Gpio::new()
        .unwrap()
        .get(GPIO_LED_PIN)
        .unwrap()
        .into_output();
    pin.set_low();
}
