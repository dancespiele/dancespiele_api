use super::{fetch_percentages, inset_percentages};
use crate::guard::auth;
use sled::Db;
use warp::{Filter, Rejection, Reply};

pub fn percentages(
    tree: impl Filter<Extract = (Db,), Error = Rejection> + Clone + Send,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("percentages").and(
        auth()
            .and(
                warp::post()
                    .and(tree.clone())
                    .and(warp::body::json())
                    .and_then(inset_percentages)
                    .or(warp::get().and(tree).and_then(fetch_percentages)),
            )
            .map(|_user, response| response),
    )
}
