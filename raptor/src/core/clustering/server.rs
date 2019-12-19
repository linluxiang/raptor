use crate::core::clustering::node;
use tokio::prelude::*;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub fn default() -> Self {
        Server {}
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        let ni = node::NodeInfo {
            addr: "0.0.0.0".to_owned(),
            port: 6666,
        };
        println!("listen: {:?}", ni.to_string());
        let mut listener = TcpListener::bind("0.0.0.0:6666").await?;
        let server = tokio::spawn(async move {
            loop {
                let (stream, socketaddr) = listener.accept().await?;
                println!("{0}", socketaddr);
            }
        });
        server.await?
    }
}