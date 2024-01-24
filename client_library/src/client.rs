use std::{any::Any, sync::Arc};

use fxhash::FxHashMap;

use crate::{
    client_config::ParsedConfig, client_polling, client_registration,
    client_types::types::DeviceCapabilityStatus,
};

//todo: make these optional parameters user can change
const RSA_KEY_SIZE: usize = 2048;
const POLLING_INTERVAL: u64 = 500;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;

pub struct ClientHandler<S: State> {
    callbacks: FxHashMap<String, Box<Callback<S>>>,
    state: S,
}

impl<S> ClientHandler<S>
where
    S: State + 'static,
{
    pub fn new() -> Self {
        Self {
            callbacks: FxHashMap::default(),
            state: S::default(),
        }
    }
    ///Replaces current callback function with supplied one
    pub fn add_callback(mut self, capabillity: &str, callback: Box<Callback<S>>) -> Self {
        self.callbacks.insert(String::from(capabillity), callback);
        return self;
    }
    pub fn add_state(mut self, state: S) -> Self {
        self.state = state;
        return self;
    }

    ///consumes self
    pub async fn run(mut self, config: ParsedConfig, server_ip: String) -> anyhow::Result<()> {
        let private_key;
        let capabilities = config.capabilities;

        let capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>> =
            Arc::new(tokio::sync::Mutex::new(capabilities));
        {
            let mut rng = rand::thread_rng();
            private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
        }

        let client_connection_details = client_registration::repeated_register_self(
            &private_key.to_public_key(),
            capabilities.clone(),
            config.device_name,
            server_ip.clone(),
        )
        .await;

        {
            let certificate = client_connection_details.security_certificate.clone();
            let uuid = client_connection_details.uuid.clone();
            let has_update = ThreadSafeMutable::new(tokio::sync::Mutex::new(false));

            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(POLLING_INTERVAL));

            let mut polling_service = client_polling::PollingService {
                client: client_polling::polling::request_update_service_client::RequestUpdateServiceClient::connect(server_ip).await.unwrap(),
                capabilities: capabilities.clone(),
                updated: has_update.clone(),
            };

            loop {
                interval.tick().await;
                let updates = polling_service
                    .get_updates(certificate.clone(), uuid.clone())
                    .await;
                let updates = match updates {
                    Some(v) => v,
                    None => {
                        // println!("No updates available");
                        continue;
                    }
                };
                for update in updates.into_iter() {
                    let callback = self.callbacks.get(&update.capability);
                    //todo: remove
                    let temp_req = Request::new();
                    match callback {
                        Some(v) => v(&mut self.state, temp_req),
                        None => println!("Received signal to {}", update.capability),
                    }
                }
            }
        }
    }
}

pub trait State: Default {}

pub type Callback<S> = fn(&mut S, Request);

pub struct Request {
    contents: Arc<String>,
}
impl Request {
    pub fn new() -> Self {
        Self {
            contents: Arc::default(),
        }
    }
}
