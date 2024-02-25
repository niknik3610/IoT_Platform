use rsa::{pss::{BlindedSigningKey, Signature, VerifyingKey}, sha2::Sha256, signature::{Keypair, RandomizedSigner, Verifier}, RsaPrivateKey};

pub struct CertificateSigningService
{
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
        return self.signing_key.sign_with_rng(&mut rng, data.as_bytes()).to_string();
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

pub fn verify_signature(verifying_key: &VerifyingKey<Sha256>, data: String, signature: String) -> bool {
        let signature = Signature::try_from(signature.as_bytes());
        let signature = match signature {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Unable to parse signature from request");
                return false;
            }
        };

        match verifying_key.verify(data.as_bytes(), &signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
