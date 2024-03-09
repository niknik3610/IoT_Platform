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
    },
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
    #[arg(long, short, default_value_t=1)]
    count: u64,
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

    println!("Starting DDOS Attack");

    for _ in 0..args.count {
        println!("finished iteration");
        let devices = get_connected_devices(&device_id, &mut registration_client).await.unwrap();
        for device in devices.iter() {
            for capability in device.capabilities.iter() {
                if capability.available {
                    control_client.control_device(DeviceControlRequest {
                        capability: capability.capability.clone(),
                        device_uuid: device.device_uuid.clone()
                    }).await.unwrap();
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
