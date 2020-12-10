use super::{create_user, get_user};
use crate::db::Pool;
use crate::guard::{auth, check_role};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

pub fn user(
    pool: impl Filter<Extract = (Arc<Mutex<Pool>>,), Error = Rejection> + Clone + Send,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("user")
        .and(
            warp::post()
                .and(warp::header::optional::<String>("Authorization"))
                .and_then(check_role)
                .and(pool.clone())
                .and(warp::body::json())
                .and_then(create_user),
        )
        .or(warp::get().and(auth()).and(pool).and_then(get_user))
}
