mod infra;

use infra::logger;

#[tokio::main]
async fn main() {
    logger::init();

    println!("Hello, world!");
}
