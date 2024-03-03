use rsa::{
    pss::{BlindedSigningKey, Signature, VerifyingKey},
    sha2::Sha256,
    signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier},
    RsaPrivateKey,
};

use crate::types::types::DeviceCapabilityStatus;

const SIGNATURE_EXPIRATION_SECONDS: u64 = 10;

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
    pub fn gen_certificate(&self, csr: &String) -> String {
        let mut rng = rand::thread_rng();
        let certificate = self
            .signing_key
            .sign_with_rng(&mut rng, csr.as_bytes())
            .to_string();
        certificate
    }
    pub fn verify_certificate(
        &self,
        certificate_from_device: String,
        certificate_authentic: String
    ) -> bool {
        let certificate_authentic = Signature::try_from(&*certificate_authentic.as_bytes());
        let certificate_authentic = match certificate_authentic {
            Ok(r) => r,
            Err(_e) => {
                eprintln!("Unable to parse signature from request");
                return false;
            }
        };

        match self.verification_key.verify(&certificate_from_device.as_bytes(), &certificate_authentic) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    ///returns the signature and timestamp of the signature
    pub fn sign_data(&self, data: String) -> (Vec<u8>, u64) {
        let timestamp = get_timestamp();
        let data = timestamp.to_string() + &data;
        let mut rng = rand::thread_rng();
        let signed = self.signing_key.sign_with_rng(&mut rng, data.as_bytes());
        println!("signed: {}", signed.to_string());
        (signed.to_vec(), timestamp)
    }

    pub fn verify_signature_update_request(
        certificate: &String,
        updated_capabilities: &Vec<DeviceCapabilityStatus>,
        client_timestamp: u64,
        signature: Vec<u8>,
        client_verification_key: &VerifyingKey<Sha256>
        ) -> bool {
        let server_timestamp = get_timestamp();

        if server_timestamp - client_timestamp > SIGNATURE_EXPIRATION_SECONDS {
            println!("Client signature has expired");
            return false;
        }

        let capability_string = updated_capabilities
            .iter()
            .map(|capability| capability.capability.clone())
            .reduce(|acc, capability| acc + &capability)
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
        match client_verification_key.verify(to_check_against.as_bytes(), &signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}



pub fn get_timestamp() -> u64 {
    use std::time::SystemTime;
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    since_epoch.as_secs()
}
