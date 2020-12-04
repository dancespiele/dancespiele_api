use super::{get_role, Claims, GetUserAuthDto, LoginDto, Role, Roles, UserDto};
use crate::db::Pool;
use crate::error::{BadRequest, DatabaseError, Forbidden, HashPwdError, JwtError};
use crate::user::{User, UserAuth, UserRole};
use bcrypt::verify;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{reject, reply, Rejection, Reply};

pub async fn login(pool: Arc<Mutex<Pool>>, login_body: LoginDto) -> Result<impl Reply, Rejection> {
    use crate::schema::role::dsl::{id as role_id, role};
    use crate::schema::user::dsl::*;

    let conn: &PgConnection = &pool.lock().await.get().unwrap();
    let secret = env::var("SECRET").expect("SECRET must be set");

    let query = if let Some(login_email) = login_body.email {
        Ok(user.filter(email.eq(login_email)).into_boxed())
    } else if let Some(login_username) = login_body.username {
        Ok(user.filter(username.eq(login_username)).into_boxed())
    } else {
        Err("At least email or username field should be set")
    }
    .map_err(|err| {
        reject::custom(BadRequest {
            error: err.to_string(),
        })
    })?;

    let user_entity = query
        .get_result::<User>(conn)
        .optional()
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    if let Some(usr) = user_entity {
        let user_auth_entity = UserAuth::belonging_to(&usr)
            .get_result::<UserAuth>(conn)
            .map_err(|err| reject::custom(DatabaseError { error: err }))?;

        let user_role_entity = UserRole::belonging_to(&usr)
            .get_result::<UserRole>(conn)
            .map_err(|err| reject::custom(DatabaseError { error: err }))?;

        let role_entity = role
            .filter(role_id.eq(user_role_entity.role_id))
            .get_result::<Role>(conn)
            .map_err(|err| reject::custom(DatabaseError { error: err }))?;

        let match_pwd = verify(login_body.password, &user_auth_entity.password)
            .map_err(|err| reject::custom(HashPwdError { error: err }))?;

        if match_pwd {
            let claims = Claims::from((usr, role_entity));

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .map_err(|err| reject::custom(JwtError { error: err }))?;

            Ok(reply::json(&token))
        } else {
            Err(reject::custom(Forbidden {
                error: "The user or password does not match".to_string(),
            }))
        }
    } else {
        Err(reject::custom(Forbidden {
            error: "The user or password does not match".to_string(),
        }))
    }
}

pub async fn auth_guard(auth_token: String) -> Result<UserDto, Rejection> {
    let secret = env::var("SECRET").expect("SECRET must be set");

    let token = decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| reject::custom(JwtError { error: err }))?;

    let user = UserDto::from(token.claims);

    Ok(user)
}

pub async fn check_role(auth_token: Option<String>) -> Result<Option<String>, Rejection> {
    let secret = env::var("SECRET").expect("SECRET must be set");

    if let Some(auth) = auth_token {
        let token = decode::<Claims>(
            &auth,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|err| reject::custom(JwtError { error: err }))?;

        Ok(Some(token.claims.role))
    } else {
        Ok(None)
    }
}

pub async fn _role_guard(
    user_auth_dto: GetUserAuthDto,
    roles_auth: Vec<Roles>,
) -> Result<GetUserAuthDto, Rejection> {
    if roles_auth
        .into_iter()
        .any(|role| get_role(role) == user_auth_dto.role)
    {
        Ok(user_auth_dto)
    } else {
        Err(reject::custom(Forbidden {
            error: "The user is not authorized to access to this resource".to_string(),
        }))
    }
}
