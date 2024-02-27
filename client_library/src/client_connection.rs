use rsa::{pss::VerifyingKey, sha2::Sha256};

pub struct ServerConnection {
    pub uuid: String,
    pub server_pub_key: rsa::RsaPublicKey,
    pub security_certificate: String,
    pub server_verifying_key: VerifyingKey<Sha256>,
}
impl ServerConnection {
    pub fn new(
        uuid: String,
        server_pub_key: rsa::RsaPublicKey,
        security_certificate: String,
    ) -> Self {
        let verifying_key = VerifyingKey::<Sha256>::from(server_pub_key.clone());
        ServerConnection {
            uuid,
            server_pub_key,
            security_certificate,
            server_verifying_key: verifying_key,
        }
    }
}
