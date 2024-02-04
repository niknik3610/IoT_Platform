//! # Nik's Open-source Smart Home Platform Client Library
//!
//! This crate is designed to serve as an easy to use API to communicate with the [NOSHP Server](https://github.com/niknik3610/IoT_Platform).
//!
//! # Example
//! This is a simple example of how to use this library. It will the server about two buttons
//! (defined in the config.toml file): "Turn On" and "Turn Off".
//! 
//! When either of these buttons are pressed, the callback assigned to them will be called, in this
//! case printing their respective actions. 
//!
//! The callback can have state assigned to it, and in the future will contain 
//! information about the user's actions. For example if they input a number into a field, this will be 
//! displayed in the request (not implemented yet).
//!
//! ```
//! use std::error::Error;
//! use NOSHP_Client::{client::{ClientHandler, Request, State}, client_config::{ClientConfig, ParsedConfig}};
//!  
//! #[derive(Default)]
//! struct ExampleState { 
//!     text: String,
//! }
//! impl State for ExampleState {}
//! 
//! const CONFIG_PATH: &str = "./example_config.toml";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let config = ClientConfig::load_config(CONFIG_PATH);
//!     let config = match config {
//!         Ok(r) => r,
//!         Err(e) => {
//!             eprintln!("Error loading config: {}", e.to_string());
//!             println!("Loading default config...");
//!             ParsedConfig::default()
//!         }
//!     };
//!
//!     let client_handler = ClientHandler::new();
//!
//!     client_handler
//!     .set_state(ExampleState { text: String::from("hello world") })
//!     .add_callback("Turn On", Box::new(turn_on_led))
//!     .add_callback("Turn Off", Box::new(turn_off_led))
//!     .run(config)
//!     .await.unwrap();
//!
//!     return Ok(());
//! }
//!
//! fn turn_on_led(state: &mut ExampleState, req: Request) {
//!     println!("turned on led");
//! }
//! fn turn_off_led(state: &mut ExampleState, req: Request) {
//!     println!("turned off led");
//! } 
//! ```
//!

pub mod client;
pub mod client_config;
pub mod client_types;

mod client_connection;
mod client_polling;
mod client_registration;
