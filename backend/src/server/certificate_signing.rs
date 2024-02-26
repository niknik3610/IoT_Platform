use rsa::{
    pkcs1::EncodeRsaPublicKey, pkcs8::LineEnding, pss::{BlindedSigningKey, Signature, VerifyingKey}, sha2::Sha256, signature::{Keypair, RandomizedSigner, Verifier}, RsaPrivateKey
};

use crate::types::types::DeviceCapabilityStatus;

pub struct CertificateSigningService {
    signing_key: BlindedSigningKey<Sha256>,
    pub verification_key: VerifyingKey<Sha256>,
}
impl CertificateSigningService {
    pub fn new(private_key: RsaPrivateKey) -> Self {
        let signing_key = BlindedSigningKey::<Sha256>::new(private_key);
        let verification_key = signing_key.verifying_key();

        Self {
            signing_key,
            verification_key,
        }
    }
    pub async fn sign_data(&self, data: String) -> String {
        let mut rng = rand::thread_rng();
        return self
            .signing_key
            .sign_with_rng(&mut rng, data.as_bytes())
            .to_string();
    }
    pub async fn verify_certificate(
        &self,
        certificate: String,
        device_pub_key: String,
        device_uuid: String,
    ) -> bool {
        let csr = device_uuid + &device_pub_key;
        let comparison_certificate = self.sign_data(csr).await;
        return certificate == comparison_certificate;
    }
}

pub fn verify_signature(
    verifying_key: &VerifyingKey<Sha256>,
    certificate: &String,
    updated_capabilities: &Vec<DeviceCapabilityStatus>,
    client_timestamp: u64,
    signature: Vec<u8>,
) -> bool {
    let server_timestamp = get_timestamp();

    if server_timestamp - client_timestamp > 60 {
        println!("Signature has expired");
        return false;
    }

    let capability_string = updated_capabilities
        .iter()
        .map(|capability| capability.capability.clone())
        .reduce(|acc, capability| {
            acc + &capability
        })
        .unwrap_or(String::from(""));

    let to_check_against = client_timestamp.to_string() + &capability_string + certificate;

    let signature = Signature::try_from(&*signature);
    let signature = match signature {
        Ok(r) => r,
        Err(_e) => {
            eprintln!("Unable to parse signature from request");
            return false;
        }
    };

    match verifying_key.verify(to_check_against.as_bytes(), &signature) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_timestamp() -> u64 {
    use std::time::SystemTime;
    let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}
