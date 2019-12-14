use tokio::net::{TcpListener, TcpStream};
// use tokio_test::*;

pub struct NodeInfo {}

pub struct Server {}

impl Server {
    pub fn default() -> Self {
        Server {}
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
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
