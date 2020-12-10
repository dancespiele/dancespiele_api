use super::{set_role, Role, Roles};
use crate::user::{GetUserDto, User};
use chrono::prelude::*;
use chrono::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub email: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginDto {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserDto {
    pub email: String,
    pub role: Roles,
}

impl From<Claims> for UserDto {
    fn from(claims: Claims) -> Self {
        Self {
            email: claims.email,
            role: set_role(&claims.role),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateServerParamDto {
    pub name: String,
}

impl From<CreateServerParamDto> for Role {
    fn from(role: CreateServerParamDto) -> Self {
        let uuid = format!("{}", uuid::Uuid::new_v4());
        Self {
            id: format!(
                "r-{}",
                uuid.parse::<String>()
                    .expect("problem to pass to String from uuid format")
            ),
            name: role.name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<GetUserDto> for Claims {
    fn from(user: GetUserDto) -> Self {
        let expire_token: DateTime<Utc> = Utc::now() + Duration::days(1);

        Self {
            sub: user.id,
            iss: user.username,
            email: user.email,
            role: user.role,
            iat: Utc::now().timestamp(),
            exp: expire_token.timestamp(),
        }
    }
}

impl From<(User, Role)> for Claims {
    fn from(user: (User, Role)) -> Self {
        let (user_entity, role_entity) = user;
        let expire_token: DateTime<Utc> = Utc::now() + Duration::days(10);

        Self {
            sub: user_entity.id,
            iss: user_entity.username,
            email: user_entity.email,
            role: role_entity.name,
            iat: Utc::now().timestamp(),
            exp: expire_token.timestamp(),
        }
    }
}

pub struct GetUserAuthDto {
    pub id: String,
    pub email: String,
    pub role: String,
}

impl From<Claims> for GetUserAuthDto {
    fn from(claims: Claims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        }
    }
}
