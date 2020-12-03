use super::models::{User, UserAuth, UserRole, UserToken};
use crate::guard::TokenType;
use crate::sql_types::StatusType;
use chrono::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<CreateUserDto> for User {
    fn from(create_user_dto: CreateUserDto) -> Self {
        let uuid = format!("{}", uuid::Uuid::new_v4());
        Self {
            id: format!(
                "u-{}",
                uuid.parse::<String>()
                    .expect("problem to pass to String from uuid format")
            ),
            username: create_user_dto.username,
            email: create_user_dto.email,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: StatusType::Inactive,
        }
    }
}

impl From<(User, String)> for GetUserDto {
    fn from(user: (User, String)) -> Self {
        let (user_entity, role_name) = user;

        Self {
            id: user_entity.id,
            username: user_entity.username,
            email: user_entity.email,
            role: role_name,
        }
    }
}

impl From<(String, String)> for UserAuth {
    fn from(user_auth: (String, String)) -> Self {
        let uuid = format!("{}", uuid::Uuid::new_v4());

        let (user_id, password) = user_auth;

        Self {
            id: format!(
                "ua-{}",
                uuid.parse::<String>()
                    .expect("problem to pass to String from uuid format")
            ),
            user_id,
            password,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<(String, String)> for UserRole {
    fn from(user: (String, String)) -> Self {
        let (user_id, role_id) = user;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        Self {
            id: format!(
                "ur-{}",
                uuid.parse::<String>()
                    .expect("problem to pass to String from uuid format")
            ),
            user_id,
            role_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<(GetUserDto, TokenType, String)> for UserToken {
    fn from(user_token: (GetUserDto, TokenType, String)) -> Self {
        let (user_entity, token_type, token) = user_token;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        Self {
            id: format!(
                "ut-{}",
                uuid.parse::<String>()
                    .expect("problem to pass to String from uuid format")
            ),
            user_id: user_entity.id,
            token_type_id: token_type.id,
            token,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
