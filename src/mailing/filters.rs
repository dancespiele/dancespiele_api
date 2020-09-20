use super::send_stop_loss_email;
use crate::guard::auth;
use warp::{Filter, Rejection, Reply};

pub fn mail() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("mail").and(
        warp::post().and(
            auth()
                .and(warp::body::json())
                .and_then(send_stop_loss_email),
        ),
    )
}
