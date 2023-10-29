pub mod client_registration;

pub const SERVER_IP: &str = "http://[::1]:50051";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let id = client_registration::repeated_register_self().await;
    println!("{id}");
    Ok(())
}
