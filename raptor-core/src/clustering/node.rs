use serde::{Deserialize, Serialize};
use serde_json;
use tokio::net::{TcpListener, TcpStream};
// use tokio_test::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub addr: String,
    pub port: u32,
}

impl NodeInfo {
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub struct Server {}

impl Server {
    pub fn default() -> Self {
        Server {}
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        let node = NodeInfo {
            addr: "0.0.0.0".to_owned(),
            port: 6666,
        };
        print!("listen: {:?}", node.to_string());
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
