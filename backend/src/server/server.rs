use std::{future::IntoFuture, net::SocketAddr, sync::{Arc, Mutex, RwLock}};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use device_control_service::frontend_device_control::frontend_device_control_service_server;
use fxhash::FxHashMap;
use local_ip_address::local_ip;
use polling::polling_service::request_update_service_server;
use registration::{
    frontend_registration_service::frontend_registration_service_server,
    registration_service::registration_service_server,
};
use rsa::pkcs1v15::SigningKey;

use futures_util::FutureExt;
use tonic::transport::Server;
use web_json_translation::json_registration;

use crate::{device::Device, polling::DeviceEvent, web_json_translation::{json_device_control, json_get_devices}};

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

pub const PERF_TEST_LOGGING: bool = true;
const RSA_KEY_SIZE: usize = 2048;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Run an additional JSON frontend api endpoint, all json requests get routed to main GRPC
    #[arg(long, default_value_t = false)]
    json_frontend: bool,
}

const DEFAULT_PORT: u16 = 2302;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    //todo: let user input a custom ip + port (idk for what reason tho)
    let local_ip = local_ip().map_err(|e| {
        eprintln!(
            "Error getting local ip address
                  You can try setting it manually using --set-ip:{{ip}}"
        );
        e
    })?;

    //todo: put this into Args
    let port = DEFAULT_PORT;
    let grpc_address = SocketAddr::new(local_ip, port);

    let connected_devices: Arc<RwLock<FxHashMap<String, Arc<Mutex<Device>>>>> = Arc::default();
    let connected_device_uuids: ThreadSafeMutable<Vec<String>> = Arc::default();

    let device_count = Arc::new(tokio::sync::Mutex::new(0));

    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
    }

    let _signing_key = SigningKey::<rsa::sha2::Sha256>::new(private_key.clone());

    let frontend_cache_valid = ThreadSafeMutable::new(tokio::sync::Mutex::new(false));
    let signing_service = Arc::new(certificate_signing::CertificateSigningService::new(private_key.clone()));

    let registration_service = registration::ClientRegistrationHandler::new(
        connected_devices.clone(),
        device_count,
        signing_service.clone(),
        private_key.to_public_key(),
        connected_device_uuids.clone(),
        frontend_cache_valid.clone(),
    );

    let events: Arc<RwLock<FxHashMap<String, Arc<Mutex<Vec<DeviceEvent>>>>>> = Arc::default();
    let polling_service = polling::PollingHandler::new(
        connected_devices.clone(),
        events.clone(),
        frontend_cache_valid.clone(),
        signing_service,
    );
    let frontend_registration_service = registration::FrontendRegistrationHandler::new(
        connected_devices.clone(),
        connected_device_uuids.clone(),
        frontend_cache_valid.clone(),
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
        .serve_with_shutdown(grpc_address, tokio::signal::ctrl_c().map(drop));

    // println!("Started GRPC Server on {grpc_address}");

    let grpc_handle = tokio::spawn(async move { grpc_server.await });

    if args.json_frontend {
        match run_json_frontend(grpc_address).await {
            Ok(json_server) => {
                tokio::spawn(async move { json_server.await });
            }
            Err(e) => {
                eprintln!("Error when starting JSON server:");
                eprintln!("{e}");
            }
        };
    };

    match grpc_handle.into_future().await {
        // Ok(_) => println!("The GRPC Server excited Succesfully"),
        Ok(_) => {},
        Err(e) => {
            eprintln!("There was an error while running the GRPC Server:");
            eprintln!("{e}");
        }
    };
    Ok(())
}

async fn run_json_frontend(grpc_address: SocketAddr) -> anyhow::Result<actix_web::dev::Server> {
    const JSON_ADDRESS: &str = "localhost:50052";
    let grpc_address = String::from("http://") + &grpc_address.to_string();

    println!("Starting JSON API Layer...");
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
    let json_state = loop {
        interval.tick().await;
        let result = web_json_translation::json_translation::TranslationClientState::new(
            grpc_address.clone(),
        )
        .await;
        match result {
            Ok(r) => break r,
            Err(e) => {
                println!("Could not connect to grpc service, retrying...");
                eprintln!("{e}");
            }
        }
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let result = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .app_data(web::Data::new(json_state.clone()))
            .wrap(cors)
            // .wrap(actix_web::middleware::Logger::default()) //todo: make optional
            .service(json_registration::json_registration)
            .service(json_registration::json_registration_options)
            .service(json_get_devices::json_get_connected_devices)
            .service(json_get_devices::json_get_connected_devices_options)
            .service(json_device_control::json_device_control)
            .service(json_device_control::json_device_control_options)
    })
    .bind(JSON_ADDRESS)?
    .run();

    println!("Successfully Started JSON API Layer on {JSON_ADDRESS}");

    return Ok(result);
}
