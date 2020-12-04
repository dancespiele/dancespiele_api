use super::dtos::{CreateUserDto, GetUserDto};
use super::models::{User, UserAuth, UserRole};
use super::set_user_role;
use crate::db::Pool;
use crate::error::{BadRequest, DatabaseError, HashPwdError};
use crate::guard::Role;
use crate::guard::{get_role, set_role, Roles, UserDto};
use bcrypt::hash;
use diesel::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{reject, reply, Rejection, Reply};

pub async fn create_user(
    role_guard: Option<String>,
    pool: Arc<Mutex<Pool>>,
    create_user_body: CreateUserDto,
) -> Result<impl Reply, Rejection> {
    use crate::schema::auth::dsl::*;
    use crate::schema::role::dsl::{name as role_name, role};
    use crate::schema::user::dsl::*;
    use crate::schema::user_role::dsl::*;

    let conn: &PgConnection = &pool.lock().await.get().unwrap();

    let user_exists = user
        .filter(
            email
                .eq(create_user_body.email.clone())
                .or(username.eq(create_user_body.username.clone())),
        )
        .get_result::<User>(conn)
        .optional()
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    if user_exists.is_some() {
        return Err(reject::custom(BadRequest {
            error: format!(
                "User with email {} or username {} already exists",
                create_user_body.email, create_user_body.username
            ),
        }));
    }

    let user_model = User::from(create_user_body.clone());

    let user_created = diesel::insert_into(user)
        .values(user_model)
        .get_result::<User>(conn)
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    let password_hashed = hash(create_user_body.password, 4)
        .map_err(|err| reject::custom(HashPwdError { error: err }))?;

    let user_auth_model = UserAuth::from((user_created.id.clone(), password_hashed));

    diesel::insert_into(auth)
        .values(user_auth_model)
        .execute(conn)
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    let role_to_set = if let Some(rg) = role_guard {
        set_user_role(set_role(&rg))
    } else {
        get_role(Roles::Customer)
    };

    let role_response = role
        .filter(role_name.eq(role_to_set.clone()))
        .get_result::<Role>(conn)
        .optional()
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    if let Some(role_model) = role_response {
        let user_role_model = UserRole::from((user_created.id.clone(), role_model.id.clone()));
        diesel::insert_into(user_role)
            .values(user_role_model)
            .execute(conn)
            .map_err(|err| reject::custom(DatabaseError { error: err }))?;
        let get_user_dto = GetUserDto::from((user_created, role_model.name));
        Ok(reply::json(&get_user_dto))
    } else {
        Err(reject::custom(BadRequest {
            error: format!("Role with name {} does not exist", role_to_set),
        }))
    }
}

pub async fn get_user(user_dto: UserDto, pool: Arc<Mutex<Pool>>) -> Result<impl Reply, Rejection> {
    use crate::schema::role::dsl::{id, role};
    use crate::schema::user::dsl::{email, user};

    let conn: &PgConnection = &pool.lock().await.get().unwrap();

    let user_model = user
        .filter(email.eq(user_dto.email))
        .get_result::<User>(conn)
        .map_err(|_| reject::not_found())?;

    let user_role_model = UserRole::belonging_to(&user_model)
        .get_result::<UserRole>(conn)
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    let role_mode = role
        .filter(id.eq(user_role_model.role_id))
        .get_result::<Role>(conn)
        .map_err(|err| reject::custom(DatabaseError { error: err }))?;

    let user_data = GetUserDto::from((user_model, role_mode.name));

    Ok(reply::json(&user_data))
}
