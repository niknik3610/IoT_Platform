pub struct ServerConnection {
    pub uuid: String,
    pub server_pub_key: rsa::RsaPublicKey,
    pub security_certificate: String,
}
impl ServerConnection {
    pub fn new(
        uuid: String,
        server_pub_key: rsa::RsaPublicKey,
        security_certificate: String,
    ) -> Self {
        return ServerConnection {
            uuid,
            server_pub_key,
            security_certificate,
        };
    }
}
