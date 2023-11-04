use rsa::{pkcs1v15::SigningKey, sha2::Sha256};

pub struct CertificateSigningService {
    signing_key: SigningKey<Sha256>,
}
impl CertificateSigningService {
    pub fn new(signing_key: SigningKey<Sha256>) -> Self {
        return CertificateSigningService { signing_key };
    }
    pub async fn sign_certificate(&self, csr: String) -> String {
        use rsa::signature::Signer;

        let signed_rsa: String = self.signing_key.sign(&csr[..].as_bytes()).to_string();
        return signed_rsa + &csr;
    }
    pub async fn verify_certificate(
        &self,
        certificate: String,
        device_pub_key: String,
        device_uuid: String,
    ) -> bool {
        let csr = device_uuid + &device_pub_key;
        let comparison_certificate = self.sign_certificate(csr).await;
        return certificate == comparison_certificate;
    }
}
