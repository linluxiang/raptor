use raptor::core::clustering::server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    server::Server::default().start().await
}
