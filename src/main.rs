#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

mod db;
mod error;
mod guard;
mod percentages;
mod schema;
mod sql_types;
mod tasks;
mod user;

use db::init_pool;
use dotenv::dotenv;
use error::error_handler;
use guard::user_login;
use percentages::percentages;
use std::env;
use std::net::SocketAddr;
use tasks::start_consumer;
use tokio::io::Result;
use user::user;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let addr: SocketAddr = server_url.parse().unwrap();
    let pool = init_pool().unwrap();

    let routes = percentages()
        .or(user(pool.clone()))
        .or(user_login(pool))
        .recover(error_handler);

    tokio::spawn(async {
        info!("Start task consumer");
        start_consumer().await.unwrap();
    });

    info!("Start server in address {}", server_url);
    warp::serve(routes).run(addr).await;

    Ok(())
}
