pub struct ServerConnection {
    pub uuid: String,
    pub server_pub_key: rsa::RsaPublicKey,
    pub security_certificate: String,
    nonce: u64,
}
impl ServerConnection {
    pub fn new(
        uuid: String,
        server_pub_key: rsa::RsaPublicKey,
        security_certificate: String,
    ) -> Self {
        ServerConnection {
            uuid,
            server_pub_key,
            security_certificate,
            nonce: 0
        }
    }
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
    pub fn increment_nonce(&mut self) {
        self.nonce += 1
    }
}
