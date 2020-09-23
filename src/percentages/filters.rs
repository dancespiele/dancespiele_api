use super::{fetch_percentages, inset_percentages};
use crate::guard::auth;
use warp::{Filter, Rejection, Reply};

pub fn percentages() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("percentages").and(
        auth()
            .and(
                warp::post()
                    .and(warp::body::json())
                    .and_then(inset_percentages)
                    .or(warp::get().and_then(fetch_percentages)),
            )
            .map(|_user, response| response),
    )
}
