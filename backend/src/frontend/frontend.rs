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
pub const SERVER_IP: &str = "http://[::1]:50051"; //todo: perhaps change this

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    loop {
        println!(
            "What would you like to do?
1. Control a device
2. Quit"
        );
        input_buffer.clear();
        {
            let stdin = std::io::stdin();
            stdin.read_line(&mut input_buffer).unwrap(); //todo
        }

        input_buffer.pop();
        let choice = match input_buffer.parse::<u32>() {
            Ok(r) => r,
            Err(e) => {
                println!("Please enter a valid input");
                eprintln!("{}", e);
                continue;
            }
        };

        match choice {
            1 => control_device(&device_id, &mut registration_client, &mut control_client).await,
            2 => break,
            _ => {
                println!("Please enter a valid input");
                continue;
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
) {
    let connected_devices = get_connected_devices(device_id, registration_client);
    println!("Fetching available devices...");
    let mut input_buffer = String::new();

    let connected_devices = connected_devices.await.unwrap(); //todo
    connected_devices
        .iter()
        .enumerate()
        .for_each(|(count, device)| {
            //todo: pretty print capabillities
            println!("{count}: {}", device.device_name);
        });

    println!("What device would you like to control?: ");
    {
        input_buffer.clear();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input_buffer).unwrap();
    }

    input_buffer.pop();
    let choice = input_buffer.parse::<usize>().unwrap();
    if choice >= connected_devices.len() {
        println!("Please input a valid choice!");
        return;
    }

    let device_to_control = &connected_devices[choice];
    println!("Heres what you can do:");
    device_to_control
        .capabilities
        .iter()
        .enumerate()
        .for_each(|(i, capabillity)| {
            let capabillity = match Capability::try_from(*capabillity) {
                Ok(r) => r,
                Err(_e) => {
                    println!("There was an error processing this capabillity");
                    return;
                }
            };
            println!("{}: {:?}", i, capabillity);
        });

    {
        input_buffer.clear();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input_buffer).unwrap();
    }

    input_buffer.pop();
    let choice = input_buffer.parse::<usize>().unwrap();

    let chosen_capabillity = Capability::try_from(device_to_control.capabilities[choice]);
    let chosen_capabillity = match chosen_capabillity {
        Ok(r) => r,
        Err(_e) => {
            println!("Please input a valid choice!");
            return;
        }
    };

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
}
