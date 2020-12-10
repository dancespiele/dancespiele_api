use diesel::{sql_query, PgConnection, RunQueryDsl};
use r2d2::{Builder, CustomizeConnection, Pool as StartPool};
use r2d2_diesel::{ConnectionManager, Error as PgError};
use sled::{Db, Result as DbResult};
use std::sync::Arc;
use std::{env, error::Error};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, Filter};

pub type Pool = StartPool<ConnectionManager<PgConnection>>;

pub type PoolBoxed = Result<BoxedFilter<(Arc<Mutex<Pool>>,)>, Box<dyn Error>>;

pub fn init_pool() -> PoolBoxed {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(url);

    let add_schema = SetSchema {};
    let builder = Builder::new();
    let build = builder.connection_customizer(Box::new(add_schema));
    let pool = build.build(manager).unwrap();

    Ok(warp::any()
        .map(move || Arc::new(Mutex::new(pool.clone())))
        .boxed())
}

#[derive(Debug)]
pub struct SetSchema;

impl CustomizeConnection<PgConnection, PgError> for SetSchema {
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), PgError> {
        let db_schema = env::var("DB_SCHEMA").expect("DB_SCHEMA must be set");

        if db_schema != "" {
            sql_query(format!("SET search_path = {}", db_schema))
                .execute(conn)
                .unwrap();
        }

        Ok(())
    }
}

pub fn init_tree() -> DbResult<Db> {
    let sled_url = env::var("SLED_URL").expect("SLED_URL must be provided");
    let tree = sled::open(&sled_url)?;

    Ok(tree)
}
