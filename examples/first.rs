#![warn(rust_2018_idioms)]

// extern crate raptor_core;

// #[cfg(feature = "full")]
// #[macro_use]
// extern crate tokio;

use raptor_core;
use std::error::Error;

use tokio;
// use tokio::net::TcpListener;
// use tokio::prelude::*;

#[tokio::main]
async fn first() -> Result<(), Box<dyn Error>> {
    // raptor_core::clustering::node::Server::default()
    //     .start()
    //     .await?
    // let listner = TcpListener::bind("127.0.0.1:8080").await?;
    // // listner.accept().await?;
    // Ok(())
    // Ok(())
    tokio::spawn(move || {
        println!("test");
    });
}
