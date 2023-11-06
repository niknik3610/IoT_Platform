pub mod client_connection;
pub mod client_registration;
pub mod client_types;

pub const SERVER_IP: &str = "http://[::1]:50051";
const RSA_KEY_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap(); //TODO
    }

    let client_connection_details =
        client_registration::repeated_register_self(&private_key.to_public_key()).await;
    Ok(())
}
