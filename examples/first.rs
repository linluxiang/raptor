use raptor::core::clustering::server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Way 1
    server::Server::default().start().await?;
    Ok(())
    // Way 2
    // match server::Server::default().start().await {
    //     Ok(()) => {
    //         Ok(())
    //     }
    //     Err(e) => {
    //         Err(e.into())
    //     }
    // }
    // Way 3
    // let r = server ::Server::default().start().await;
    // match r {
    //     OK(k) => {
    //         Ok(k.into())
    //     }
    // }
}
