#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub email: String,
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
