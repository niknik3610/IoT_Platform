use std::{sync::Arc, time::Duration, any::Any};

use device_control_service::frontend_device_control::frontend_device_control_service_server;
use fxhash::FxHashMap;
use polling::polling_service::request_update_service_server;
use registration::{
    frontend_registration_service::frontend_registration_service_server,
    registration_service::registration_service_server,
};
use tonic::transport::Server;
use zeroconf::{service::TMdnsService, txt_record::TTxtRecord, event_loop::{self, TEventLoop}};

pub mod certificate_signing;
pub mod device;
pub mod device_control_service;
pub mod frontend_clients;
pub mod polling;
mod registration;
pub mod types;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;
pub type RPCFunctionResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type ConnectedDevicesType = FxHashMap<String, device::Device>;

const RSA_KEY_SIZE: usize = 2048;
const ZERO_CONF_SERVICE_NAME: &str = "IOT_HUB_SERVICE";
const ZERO_CONF_SERVICE_PROTOCOL: &str = "tcp";
const ZERO_CONF_SERVICE_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;

    let connected_devices: ThreadSafeMutable<ConnectedDevicesType> = Arc::default();
    let connected_device_uuids: ThreadSafeMutable<Vec<String>> = Arc::default();

    let device_count = Arc::new(tokio::sync::Mutex::new(0));

    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
    }

    let signing_key = rsa::pkcs1v15::SigningKey::<rsa::sha2::Sha256>::new(private_key.clone());

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

    let _forever = tokio::task::spawn( async {    
        let _zeroconf_service = zero_conf_advertise_service().await;
    });

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
    Ok(())
}

async fn zero_conf_advertise_service() -> anyhow::Result<()>{
    let service_type = zeroconf::ServiceType::new(ZERO_CONF_SERVICE_NAME, ZERO_CONF_SERVICE_PROTOCOL)?;
    let mut service = zeroconf::MdnsService::new(service_type, ZERO_CONF_SERVICE_PORT);
    let mut record = zeroconf::TxtRecord::new();
    record.insert("foo", "bar")?;

    service.set_txt_record(record);
    service.set_registered_callback(Box::new(on_service_registered));
    let event_loop = service.register().unwrap();
    loop {
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
    // return Ok(());
}

fn on_service_registered(
    result: zeroconf::Result<zeroconf::ServiceRegistration>,
    _context: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();
    println!("Service registered: {:?}", service);
}
