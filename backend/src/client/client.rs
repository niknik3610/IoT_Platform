use client_config::ClientConfig;
use iot_client::{ClientHandler, State, Request};
use rppal::gpio::Gpio;

use crate::client_config::ParsedConfig;

pub mod client_config;
pub mod client_connection;
pub mod client_polling;
pub mod client_registration;
pub mod client_types;
pub mod iot_client;

#[derive(Default)]
struct ExampleState {
    text: String,
}
impl State for ExampleState {}

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

    let client_handler = ClientHandler::new();
    client_handler
        .add_state(ExampleState { text: String::from("hello world") })
        .add_callback("Turn On", Box::new(turn_on_led))
        .add_callback("Turn Off", Box::new(turn_off_led))
        .run(config).await.unwrap();
    return Ok(());
}

const GPIO_LED_PIN: u8 = 2;
fn turn_on_led(state: &mut ExampleState, req: Request) {
    let mut pin = Gpio::new()
        .unwrap()
        .get(GPIO_LED_PIN)
        .unwrap()
        .into_output();
    pin.set_high();
}

fn turn_off_led(state: &mut ExampleState, req: Request) {
    let mut pin = Gpio::new()
        .unwrap()
        .get(GPIO_LED_PIN)
        .unwrap()
        .into_output();
    pin.set_low();
}
