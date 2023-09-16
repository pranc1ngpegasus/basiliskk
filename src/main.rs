mod adapter;
mod domain;
mod infra;

use adapter::listener::ListenerImpl;
use domain::listener::Listener;
use infra::logger;

#[tokio::main]
async fn main() {
    logger::init();

    ListenerImpl::new(1178).listen().await.unwrap()
}
