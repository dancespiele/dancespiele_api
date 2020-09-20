use sled::Db;
use std::env;
use std::error::Error;
use warp::{filters::BoxedFilter, Filter};

pub fn init_tree() -> Result<BoxedFilter<(Db,)>, Box<dyn Error>> {
    let sled_url = env::var("SLED_URL").expect("SLED_URL must be provided");
    let tree = sled::open(&sled_url)?;

    Ok(warp::any().map(move || tree.clone()).boxed())
}
