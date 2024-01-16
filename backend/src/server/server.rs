use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use clap::Parser;
use device_control_service::frontend_device_control::frontend_device_control_service_server;
use fxhash::FxHashMap;
use polling::polling_service::request_update_service_server;
use registration::{
    frontend_registration_service::frontend_registration_service_server,
    registration_service::registration_service_server,
};
use rsa::pkcs1v15::SigningKey;
use tonic::transport::Server;
use web_json_translation::json_registration;

pub mod certificate_signing;
pub mod device;
pub mod device_control_service;
pub mod frontend_clients;
pub mod polling;
mod registration;
pub mod types;
mod web_json_translation;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;
pub type RPCFunctionResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type ConnectedDevicesType = FxHashMap<String, device::Device>;

const RSA_KEY_SIZE: usize = 2048;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Run an additional JSON frontend api endpoint, all json requests get routed to main GRPC
    #[arg(long, default_value_t = false)]
    json_frontend: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let addr = "[::1]:50051".parse()?;

    let connected_devices: ThreadSafeMutable<ConnectedDevicesType> = Arc::default();
    let connected_device_uuids: ThreadSafeMutable<Vec<String>> = Arc::default();

    let device_count = Arc::new(tokio::sync::Mutex::new(0));

    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
    }

    let signing_key = SigningKey::<rsa::sha2::Sha256>::new(private_key.clone());

    let signing_service = certificate_signing::CertificateSigningService::new(signing_key);
    let registration_service = registration::ClientRegistrationHandler::new(
        connected_devices.clone(),
        device_count,
        signing_service,
        private_key.to_public_key(),
        connected_device_uuids.clone(),
    );

    let events = ThreadSafeMutable::default();
    let cache_valid = ThreadSafeMutable::new(tokio::sync::Mutex::new(false));
    let polling_service = polling::PollingHandler::new(
        connected_devices.clone(),
        events.clone(),
        cache_valid.clone(),
    );
    let frontend_registration_service = registration::FrontendRegistrationHandler::new(
        connected_devices.clone(),
        connected_device_uuids.clone(),
        cache_valid.clone(),
    );
    let device_control_service =
        device_control_service::FrontendDeviceControlHandler::new(events.clone());

    let json_server_handle = if args.json_frontend {
        Some(tokio::spawn(async {
            let result = run_json_frontend().await;
            match result {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error when running JSON translation service:");
                    eprintln!("{e}");
                }
            }
        }))
    } else {
        None
    };

    Server::builder()
        .add_service(registration_service_server::RegistrationServiceServer::new(
            registration_service,
        ))
        .add_service(
            request_update_service_server::RequestUpdateServiceServer::new(polling_service),
        )
        .add_service(
            frontend_registration_service_server::FrontendRegistrationServiceServer::new(
                frontend_registration_service,
            ),
        )
        .add_service(
            frontend_device_control_service_server::FrontendDeviceControlServiceServer::new(
                device_control_service,
            ),
        )
        .serve(addr)
        .await?;

    if let Some(handle) = json_server_handle {
        let _ = handle.await;
    }
    Ok(())
}

async fn run_json_frontend() -> anyhow::Result<actix_web::dev::Server> {
    const JSON_PORT: u16 = 50052;
    const JSON_IP: &str = "127.0.0.1";

    let json_state = web_json_translation::json_translation::TranslationClientState::new().await?;

    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(json_state.clone()))
            .service(json_registration::json_registration)
    })
    .bind((JSON_IP, JSON_PORT))?
    .run();

    return Ok(result);
}
