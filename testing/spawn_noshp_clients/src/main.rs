use std::error::Error;

use clap::Parser;
use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig, ParsedConfig},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of clients to spawn
    #[arg(short, long, default_value_t = 1)]
    count: u128,
}

#[derive(Default)]
struct ExampleState {
}
impl UserDefinedState for ExampleState {}

const CONFIG_PATH: &str = "./example_config.toml";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = ClientConfig::load_config(CONFIG_PATH);
    let config = match config {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading config: {}", e.to_string());
            println!("Loading default config...");
            ParsedConfig::default()
        }
    };

    let mut thread_handles = vec![];
        // .run(config)
        // .await
        // .unwrap();
    for i in 0..args.count {
        let mut config = config.clone();
        config.device_name += &i.to_string();
        let handle = tokio::spawn(async move {
            let client_handler = NoshpClient::new()
                .add_callback("Turn On", Box::new(turn_on_led))
                .add_callback("Turn Off", Box::new(turn_off_led))
                .run(config)
                .await
                .unwrap();
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.await.unwrap();
    }
    return Ok(());
}

fn turn_on_led(_state: &mut ClientState<ExampleState>, _req: Request) {
    println!("turned on led");
}

fn turn_off_led(_state: &mut ClientState<ExampleState>, _req: Request) {
    println!("turned off led");
}
