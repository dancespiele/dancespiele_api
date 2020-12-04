use super::{auth_guard, login, UserDto};
use crate::db::Pool;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

pub fn auth() -> impl Filter<Extract = (UserDto,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(auth_guard)
}

pub fn user_login(
    pool: impl Filter<Extract = (Arc<Mutex<Pool>>,), Error = Rejection> + Clone + Send,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login").and(
        warp::post()
            .and(pool)
            .and(warp::body::json())
            .and_then(login),
    )
}
