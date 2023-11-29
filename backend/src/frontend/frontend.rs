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
    },
    frontend_types::types::Capability,
};

pub mod frontend_device_control;
pub mod frontend_registration;
pub mod frontend_types;
//todo: switch this to automatically discover servers on the network
pub const SERVER_IP: &str = "http://[::1]:50051";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //todo: add ability to exit any page on frontend
    println!("Connecting...");
    let mut registration_client = FrontendRegistrationServiceClient::connect(SERVER_IP).await?;
    let mut control_client = FrontendDeviceControlServiceClient::connect(SERVER_IP).await?;

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
        let mut choice_iterator = device_to_control
            .capabilities
            .iter()
            .enumerate()
            .peekable();

        while let Some((count, capabillity)) = choice_iterator.next() { 
            let capabillity = match Capability::try_from(*capabillity) {
                Ok(r) => r,
                Err(_e) => {
                    println!("There was an error processing this capabillity");
                    continue;
                }
            };
            println!("{}: {:?}", count, capabillity);

            if choice_iterator.peek().is_none() {
                quit_number = count + 1;
                println!("{quit_number}: Quit");
            }
        };

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

        let choice = &device_to_control.capabilities[choice]; 
        chosen_capabillity = match Capability::try_from(*choice) {
            Ok(r) => r,
            Err(_e) => {
                println!("Please enter a valid input");
                continue;
            }
        };
        break;
    }

    println!("Making request....");
    let result = control_client
        .control_device(DeviceControlRequest {
            device_uuid: device_to_control.device_uuid.clone(),
            capability: chosen_capabillity as i32,
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
