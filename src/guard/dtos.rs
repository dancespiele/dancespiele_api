use crate::user::GetUserDto;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub email: String,
}

impl From<Claims> for UserDto {
    fn from(claims: Claims) -> Self {
        Self {
            email: claims.email,
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
