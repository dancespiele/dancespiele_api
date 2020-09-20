#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod db;
mod error;
mod guard;
mod mailing;
mod percentages;

use db::init_tree;
use dotenv::dotenv;
use error::error_handler;
use mailing::mail;
use percentages::percentages;
use std::env;
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main(core_threads = 2)]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let addr: SocketAddr = server_url.parse().unwrap();
    let pool_tree = init_tree().unwrap();

    let routes = percentages(pool_tree).or(mail()).recover(error_handler);

    info!("stat server in address {}", server_url);
    warp::serve(routes).run(addr).await;
}
