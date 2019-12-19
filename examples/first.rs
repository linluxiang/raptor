use raptor::core::clustering::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::Server::default().start().await?;
    Ok(())
}
