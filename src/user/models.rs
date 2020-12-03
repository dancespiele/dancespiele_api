use crate::guard::{Role, TokenType};
use crate::schema::{auth, user, user_role, user_token};
use crate::sql_types::StatusType;
use chrono::prelude::*;

#[derive(Deserialize, Identifiable, Serialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: StatusType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Insertable, Serialize, Queryable, Deserialize, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "auth"]
pub struct UserAuth {
    pub id: String,
    pub user_id: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Insertable, Serialize, Queryable, Deserialize, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(TokenType, foreign_key = "token_type_id")]
#[table_name = "user_token"]
pub struct UserToken {
    pub id: String,
    pub user_id: String,
    pub token_type_id: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Insertable, Serialize, Queryable, Deserialize, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[table_name = "user_role"]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
