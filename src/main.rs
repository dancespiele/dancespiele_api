#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod db;
mod error;
mod guard;
mod percentages;
mod tasks;

use dotenv::dotenv;
use error::error_handler;
use percentages::percentages;
use std::env;
use std::net::SocketAddr;
use tasks::start_consumer;
use tokio::io::Result;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let addr: SocketAddr = server_url.parse().unwrap();

    let routes = percentages().recover(error_handler);

    tokio::spawn(async {
        info!("Start task consumer");
        start_consumer().await.unwrap();
    });

    info!("Start server in address {}", server_url);
    warp::serve(routes).run(addr).await;

    Ok(())
}
