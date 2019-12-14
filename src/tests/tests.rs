use raptor_core::clustering;
use tokio;
use tokio::prelude::*;
// #[test]
#[tokio::test]
async fn start_server() -> Result<(), std::io::Error> {
    // let mut rt = Runtime::new().unwrap();

    // rt.block_on(fut().unit_error().boxed().compat()).unwrap();
    clustering::node::Server::default().start().await
    // tokio::run(Server::default().start());
}

#[test]
fn test() {
    assert_eq!(1, 1);
}
