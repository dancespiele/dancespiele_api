use super::Percentage;
use crate::db::init_tree;
use crate::error::{ConvertToString, TransformError, TreeError};
use std::str;
use warp::{reject, reply, Rejection, Reply};

pub async fn inset_percentages(percentages: Vec<Percentage>) -> Result<impl Reply, Rejection> {
    let tree = init_tree().map_err(|err| reject::custom(TreeError { error: err }))?;

    let percentages_string = serde_json::to_string(&percentages)
        .map_err(|err| reject::custom(TransformError { error: err }))?;

    tree.insert("percentages", percentages_string.as_bytes())
        .map_err(|err| reject::custom(TreeError { error: err }))?;

    Ok(reply::json(&percentages))
}

pub async fn fetch_percentages() -> Result<impl Reply, Rejection> {
    let tree = init_tree().map_err(|err| reject::custom(TreeError { error: err }))?;
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
