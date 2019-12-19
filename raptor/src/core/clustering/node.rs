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
