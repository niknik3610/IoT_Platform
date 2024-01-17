use std::{sync::Arc, future::IntoFuture};

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
use web_json_translation::{json_registration, json_translation::TranslationClientState};

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

    let grpc_server = Server::builder()
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
        .serve(addr);

    let grpc_handle = tokio::spawn(async move {
        grpc_server.await
    });

    if args.json_frontend {
        match run_json_frontend().await {
            Ok(json_server) => {
                tokio::spawn(async move {
                    json_server.await
                });
            },
            Err(e) => {
                eprintln!("Error when starting JSON server:");
                eprintln!("{e}");
            }
        };
    }
    match grpc_handle.into_future().await {
        Ok(_) => println!("The GRPC Server excited Succesfully"),
        Err(e) => { 
            eprintln!("There was an error while running the GRPC Server:");
            eprintln!("{e}");
        }
    }
    Ok(())
}

async fn run_json_frontend() -> anyhow::Result<actix_web::dev::Server> {
    const ADDRESS: &str = "localhost:50052";
    println!("Starting JSON API Layer...");
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
    let json_state = loop {
        interval.tick().await;
        let result = web_json_translation::json_translation::TranslationClientState::new().await;
        match result {
            Ok(r) => break r,
            Err(_e) => println!("Could not connect to grpc service, retrying..."),
        }
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(json_state.clone()))
            .wrap(actix_web::middleware::Logger::default()) //todo: make optional
            .service(json_registration::json_registration)
    })
    .bind(ADDRESS)?
    .run();

    println!("Successfully Started JSON API Layer on {ADDRESS}");
    
    return Ok(result);
}
