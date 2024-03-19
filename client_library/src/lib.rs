//! # Nik's Open-source Smart Home Platform Client Library
//!
//! This crate is designed to serve as an easy to use API to communicate with the [NOSHP Server](https://github.com/niknik3610/IoT_Platform).
//!
//! # Example
//! This is a simple example of how to use this library. It will the server about two buttons
//! (defined in the config.toml file): "Turn On" and "Turn Off". In addition it also shows how a
//! slider can be used.
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
//! use NOSHP_Client::{
//!     client::{ClientState, NoshpClient, Request, UserDefinedState},
//!     client_config::{ClientConfig, ParsedConfig},
//! };
//! 
//! #[derive(Default)]
//! struct ExampleState {
//!     pub text: String,
//!     pub current_brightness_lvl: u32
//! }
//! impl UserDefinedState for ExampleState {}
//! 
//! const CONFIG_PATH: &str = "./example_config.toml";
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let config = ClientConfig::load_config(CONFIG_PATH).unwrap();
//! 
//!     let client_handler = NoshpClient::new();
//!     client_handler
//!         .set_state(ExampleState {
//!             text: String::from("hello world"),
//!             current_brightness_lvl: 50,
//!         })
//!         .add_callback("Turn On", Box::new(turn_on_led))
//!         .add_callback("Turn Off", Box::new(turn_off_led))
//!         .add_callback("Brightness", Box::new(handle_brightness))
//!         .run(config)
//!         .await
//!         .unwrap();
//! 
//!     return Ok(());
//! }
//! 
//! fn turn_on_led(state: &mut ClientState<ExampleState>, _req: Request) {
//!     state.update_capability_availabillity("Turn On", false).unwrap();
//!     state.update_capability_availabillity("Turn Off", true).unwrap();
//!     state.user_state.text = String::from("Turned On");
//! 
//!     println!("State: {}", state.user_state.text);
//! }
//! 
//! fn turn_off_led(state: &mut ClientState<ExampleState>, _req: Request) {
//!     state.update_capability_availabillity("Turn On", true).unwrap();
//!     state.update_capability_availabillity("Turn Off", false).unwrap();
//!     state.user_state.text = String::from("Turned Off");
//!     println!("State: {}", state.user_state.text);
//! }
//! 
//! fn handle_brightness(state: &mut ClientState<ExampleState>, req: Request) {
//!     let new_brightness = req.value.unwrap();
//!     state.user_state.current_brightness_lvl = new_brightness as u32;
//!     println!("New brightness set: {}", state.user_state.current_brightness_lvl);
//! }
//!

pub mod client;
pub mod client_config;
pub mod client_types;

mod client_connection;
mod client_polling;
mod client_registration;
