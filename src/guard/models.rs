use crate::schema::{role, token_type};
use chrono::prelude::*;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "role"]
pub struct Role {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "token_type"]
pub struct TokenType {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
