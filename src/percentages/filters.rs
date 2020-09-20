use super::inset_percentages;
use crate::guard::auth;
use sled::Db;
use warp::{Filter, Rejection, Reply};

pub fn percentages(
    tree: impl Filter<Extract = (Db,), Error = Rejection> + Clone + Send,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("percentages").and(
        warp::post()
            .and(auth())
            .and(tree)
            .and(warp::body::json())
            .and_then(|_user, tree, body| inset_percentages(tree, body)),
    )
}
