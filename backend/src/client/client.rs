pub mod client_registration;

pub const SERVER_IP: &str = "http://[::1]:50051";
const RSA_KEY_SIZE: usize = 2048;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key;
    let public_key;
    {
        let mut rng = rand::thread_rng();
        private_key = rsa::RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE).unwrap();//TODO
        public_key = rsa::RsaPublicKey::from(private_key);
    }

    let _id = client_registration::repeated_register_self(&public_key).await;
    // println!("{id}");
    Ok(())
}
