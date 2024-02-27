use std::sync::{Arc};

use anyhow::anyhow;
use fxhash::FxHashMap;
use rsa::{
    pss::{BlindedSigningKey},
    sha2::Sha256,
};

use crate::{
    client_config::ParsedConfig, client_polling, client_registration,
    client_types::types::DeviceCapabilityStatus,
};

//todo: make these optional parameters user can change
const RSA_KEY_SIZE: usize = 2048;
const POLLING_INTERVAL: u64 = 500;

pub type ThreadSafeMutable<T> = Arc<tokio::sync::Mutex<T>>;

pub struct NoshpClient<S: UserDefinedState> {
    callbacks: FxHashMap<String, Box<Callback<S>>>,
    client_state: ClientState<S>,
    server_ip: Option<String>,
}
//state should return an object which has:
//  - user defined state
//  - all capabilities with their properties

impl<S> NoshpClient<S>
where
    S: UserDefinedState + 'static,
{
    pub fn new() -> Self {
        Self {
            callbacks: FxHashMap::default(),
            client_state: ClientState::new(),
            server_ip: None,
        }
    }
    ///Replaces current callback function with supplied one
    pub fn add_callback(mut self, capabillity: &str, callback: Box<Callback<S>>) -> Self {
        self.callbacks.insert(String::from(capabillity), callback);
        self
    }
    ///Sets the state that is passed as a parameter to callback functions
    pub fn set_state(mut self, state: S) -> Self {
        self.client_state.user_state = state;
        self
    }
    ///Overrides any server_ip set in the config, should be in format http://serverip:port
    pub fn set_server_ip(mut self, server_ip: String) -> Self {
        self.server_ip = Some(server_ip);
        self
    }
    ///Consumes self
    pub async fn run(mut self, config: ParsedConfig) -> anyhow::Result<()> {
        //this is not pretty, probably refactor in the future
        if let (None, None) = (self.server_ip.clone(), config.server_ip.clone()) {
            return Err(anyhow!("Did not receive a server ip from the config or using set_server_ip, one of them needs to be set"));
        }

        //unwrap is guaranteed to succeed due to above check
        let server_ip = match self.server_ip {
            Some(v) => v,
            None => config.server_ip.unwrap(),
        };

        let capabilities = config.capabilities;

        let capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>> =
            Arc::new(tokio::sync::Mutex::new(capabilities));

        self.client_state.capabilities = capabilities.clone();
        let (private_key, signing_key) = {
            let mut rng = rand::thread_rng();
            let private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();
            let signing_key = BlindedSigningKey::<Sha256>::new(private_key.clone());
            (private_key, signing_key)
        };

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

            let update_client = client_polling::polling::request_update_service_client
                ::RequestUpdateServiceClient::connect(server_ip).await?;

            let mut polling_service = client_polling::PollingService::new(
                update_client,
                capabilities.clone(),
                has_update.clone(),
                client_connection_details,
                signing_key,
            );

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
                    //todo: the requests aren't really implemented yet
                    let empty_request = Request::new();
                    match callback {
                        Some(callback) => callback(&mut self.client_state, empty_request),
                        None => println!("Received signal to {}", update.capability),
                    }
                }
            }
        }
    }
}

pub trait UserDefinedState: Default {}
//todo: add capabilities to the capabilities struct here
pub struct ClientState<S: UserDefinedState> {
    capabilities: ThreadSafeMutable<Vec<DeviceCapabilityStatus>>,
    user_state: S,
}
impl<S: UserDefinedState> ClientState<S> {
    fn new() -> Self {
        return Self {
            capabilities: Arc::default(),
            user_state: S::default(),
        };
    }
}

pub type Callback<S> = fn(&mut ClientState<S>, Request);

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
