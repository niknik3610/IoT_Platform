use crate::frontend_registration::registration_service::{
    frontend_registration_service_client::FrontendRegistrationServiceClient,
    ConnectedDevicesRequest, RegistrationRequest,
};

pub mod frontend_registration;
pub mod frontend_types;
pub const SERVER_IP: &str = "http://[::1]:50051"; //todo: perhaps change this

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Connecting...");
    let mut client = FrontendRegistrationServiceClient::connect(SERVER_IP).await?;
    let response = client
        .register(RegistrationRequest {
            device_name: String::from("Niks System"),
        })
        .await?;

    let response = response.into_inner();
    let device_id = response.client_id;

    println!("Connection Successful");
    println!("Welcome to Nik's Smart Home System");
    println!("Your Device id is: {device_id}");

    let connected_devices = client
        .get_connected_devices(ConnectedDevicesRequest {
            client_id: device_id,
        })
        .await?
        .into_inner()
        .devices;

    connected_devices.iter().for_each(|device| {
        println!("{:?}", device);
    });

    return Ok(());
}
