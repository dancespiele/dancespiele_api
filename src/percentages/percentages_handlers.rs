use super::Percentage;
use crate::error::{ConvertToString, TransformError, TreeError};
use sled::Db;
use std::str;
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

pub async fn fetch_percentages(tree: Db) -> Result<impl Reply, Rejection> {
    let percentages = tree
        .get("percentages")
        .map_err(|err| reject::custom(TreeError { error: err }))?
        .ok_or_else(reject::not_found)?;

    let percentages_string = str::from_utf8(&percentages)
        .map_err(|err| reject::custom(ConvertToString { error: err }))?;

    let response: Vec<Percentage> = serde_json::from_str(percentages_string)
        .map_err(|err| reject::custom(TransformError { error: err }))?;

    Ok(reply::json(&response))
}
