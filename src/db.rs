use sled::{Db, Error};
use std::env;

pub fn init_tree() -> Result<Db, Error> {
    let sled_url = env::var("SLED_URL").expect("SLED_URL must be provided");
    let tree = sled::open(&sled_url)?;

    Ok(tree)
}
