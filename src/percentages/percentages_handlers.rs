use super::Percentage;
use crate::error::{TransformError, TreeError};
use sled::Db;
use warp::{reject, reply, Rejection, Reply};

pub async fn inset_percentages(
    tree: Db,
    percentages: Vec<Percentage>,
) -> Result<impl Reply, Rejection> {
    let percentages_string = serde_json::to_string(&percentages)
        .map_err(|err| reject::custom(TransformError { error: err }))?;

    tree.insert("percentages", percentages_string.as_bytes())
        .map_err(|err| reject::custom(TreeError { error: err }))?;

    Ok(reply::json(&percentages))
}
