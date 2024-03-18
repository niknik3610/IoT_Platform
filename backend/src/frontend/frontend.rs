use clap::Parser;
use frontend_registration::registration_service::Device;
use tonic::transport::Channel;

use crate::{
    frontend_device_control::frontend_device_control::{
        frontend_device_control_service_client::FrontendDeviceControlServiceClient,
        DeviceControlRequest,
    },
    frontend_registration::registration_service::{
        frontend_registration_service_client::FrontendRegistrationServiceClient,
        ConnectedDevicesRequest, RegistrationRequest,
    }, frontend_types::types::DeviceCapabilityType,
};

pub mod frontend_device_control;
pub mod frontend_registration;
pub mod frontend_types;
//todo: switch this to automatically discover servers on the network
//todo: take this from command line arg

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Run an additional JSON frontend api endpoint, all json requests get routed to main GRPC
    #[arg(long, short, default_value_t=String::from(""))]
    server_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.server_address == "" {
        return Err(anyhow::anyhow!(
            "Error: You must include the Server's address using -s {{serverAddress}}:{{port}}"
        ));
    }
    let server_address = if !args.server_address.starts_with("https://") {
        String::from("https://") + &args.server_address
    } else {
        args.server_address
    };

    println!("Connecting...");
    let mut registration_client =
        FrontendRegistrationServiceClient::connect(server_address.clone()).await?;
    let mut control_client =
        FrontendDeviceControlServiceClient::connect(server_address.clone()).await?;

    let response = registration_client
        .register(RegistrationRequest {
            device_name: String::from("Niks System"),
        })
        .await?;

    let response = response.into_inner();
    let device_id = response.client_id;

    println!("Connection Successful");
    println!("Welcome to Nik's Smart Home System");
    println!("Your Device id is: {device_id}");

    let mut input_buffer = String::new();
    'main: loop {
        'sub: loop {
            println!(
                "\nWhat would you like to do?\n{}\n{}",
                "1. Control a device", "2. Quit"
            );

            input_buffer.clear();
            {
                let stdin = std::io::stdin();
                match stdin.read_line(&mut input_buffer) {
                    Ok(_) => {}
                    Err(_e) => {
                        println!("Please enter a valid input");
                        continue 'sub;
                    }
                }
            }
            //todo: check if this works when user selects 10
            input_buffer.pop();

            let choice = match input_buffer.parse::<u32>() {
                Ok(r) => r,
                Err(_e) => {
                    println!("Please enter a valid input");
                    continue 'sub;
                }
            };

            match choice {
                1 => {
                    control_device(&device_id, &mut registration_client, &mut control_client)
                        .await
                        .unwrap_or_else(|e| {
                            println!("Something went wrong during your request: ");
                            eprintln!("{e}");
                        });
                }
                2 => break 'main,
                _ => {
                    println!("Please enter a valid input");
                    continue 'sub;
                }
            }
        }
    }

    return Ok(());
}

async fn get_connected_devices(
    device_id: &String,
    client: &mut FrontendRegistrationServiceClient<Channel>,
) -> anyhow::Result<Vec<Device>> {
    //todo: move this out of registration, add a struct for the ConnectedDevicesClient with methods
    let connected_devices = client
        .get_connected_devices(ConnectedDevicesRequest {
            client_id: device_id.clone(),
        })
        .await?
        .into_inner()
        .devices;

    Ok(connected_devices)
}

//This code disgusts me :|
async fn control_device(
    device_id: &String,
    registration_client: &mut FrontendRegistrationServiceClient<Channel>,
    control_client: &mut FrontendDeviceControlServiceClient<Channel>,
) -> anyhow::Result<()> {
    let connected_devices = get_connected_devices(device_id, registration_client);
    println!("Fetching available devices...");
    let mut input_buffer = String::new();

    let connected_devices = connected_devices.await?;

    let mut quit_number = 99;
    let mut device_iterator = connected_devices.iter().enumerate().peekable();
    while let Some((count, device)) = device_iterator.next() {
        println!("{count}: {}", device.device_name);
        if device_iterator.peek().is_none() {
            quit_number = count + 1;
            println!("{quit_number}: Quit");
            break;
        }
    }

    let device_to_control;
    let chosen_capabillity;
    loop {
        println!("\nWhat device would you like to control?: ");
        {
            input_buffer.clear();
            let stdin = std::io::stdin();
            match stdin.read_line(&mut input_buffer) {
                Ok(_) => {}
                Err(_e) => {
                    println!("Please enter a valid input");
                    continue;
                }
            }
        }

        input_buffer.pop();
        let choice = match input_buffer.parse::<usize>() {
            Ok(r) => r,
            Err(_e) => {
                println!("Please enter a valid input");
                continue;
            }
        };
        if choice != quit_number && choice >= connected_devices.len() {
            println!("Please enter a valid input");
            continue;
        }
        if choice == quit_number {
            return Ok(());
        }
        device_to_control = &connected_devices[choice];
        break;
    }

    loop {
        println!("\nHeres what you can do:");
        let mut choice_iterator_capabilities =
            device_to_control.capabilities.iter().enumerate().peekable();

        while let Some((count, capabillity)) = choice_iterator_capabilities.next() {
            println!("{}: {}", count, capabillity.capability);

            if choice_iterator_capabilities.peek().is_none() {
                quit_number = count + 1;
                println!("{quit_number}: Quit");
            }
        }

        {
            input_buffer.clear();
            let stdin = std::io::stdin();
            match stdin.read_line(&mut input_buffer) {
                Ok(_) => {}
                Err(_e) => {
                    println!("Please enter a valid input");
                    continue;
                }
            }
        }

        input_buffer.pop();
        let choice = match input_buffer.parse::<usize>() {
            Ok(r) => r,
            Err(_e) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        if choice != quit_number && choice >= device_to_control.capabilities.len() {
            println!("Please enter a valid input");
            continue;
        }

        if choice == quit_number {
            return Ok(());
        }

        chosen_capabillity = &device_to_control.capabilities[choice];
        break;
    }
        
    let value = if chosen_capabillity.r#type == DeviceCapabilityType::Slider as i32 {
        let mut input_buffer = String::new();

        let choice = loop {
            println!("Enter a value between 0-100");
            {
                let stdin = std::io::stdin();
                match stdin.read_line(&mut input_buffer) {
                    Ok(_) => {}
                    Err(_e) => {
                        println!("Please enter a valid input");
                        continue;
                    }
                }
            }
            input_buffer.pop();
            let choice = match input_buffer.parse::<usize>() {
                Ok(r) => r,
                Err(_e) => {
                    println!("Please enter a valid input");
                    continue;
                }
            };
            //choice cannot be < 0 bc it is usize
            if  choice > 100 {
                println!("Please enter a valid input");
                continue;
            }
            break choice;
        };

        Some(choice as f32)
    } else {
        None
    };
    
    println!("Making request....");
    let result = control_client
        .control_device(DeviceControlRequest {
            device_uuid: device_to_control.device_uuid.clone(),
            capability: chosen_capabillity.capability.clone(),
            timestamp: get_timestamp(),
            value
        })
        .await;

    match result {
        Ok(_r) => println!("Operation was successful"),
        Err(e) => {
            println!("There was an error: ");
            eprintln!("{}", e.to_string());
        }
    }

    return Ok(());
}

pub fn get_timestamp() -> u64 {
    use std::time::SystemTime;
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    since_epoch.as_millis() as u64
}
