use super::{auth_guard, UserDto};
use warp::{Filter, Rejection};

pub fn auth() -> impl Filter<Extract = (UserDto,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(auth_guard)
}
