use super::{
    create_auth, create_role, create_token_type, create_user, create_user_role, create_user_token,
};
use refinery::{Migration, Runner};
use std::env;

pub enum ExecOpt {
    Up,
    Down,
}

pub struct RollbackScript {
    pub name: String,
    pub sql: String,
}

pub fn get_schema() -> String {
    env::var("SCHEMA").unwrap_or_else(|_| String::from(""))
}

pub fn get_migrations() -> Runner {
    let v1_migration =
        Migration::unapplied("V1__create_user", &create_user::migration(ExecOpt::Up)).unwrap();

    let v2_migration =
        Migration::unapplied("V2__create_auth", &create_auth::migration(ExecOpt::Up)).unwrap();

    let v3_migration = Migration::unapplied(
        "V3__create_token_type",
        &create_token_type::migration(ExecOpt::Up),
    )
    .unwrap();

    let v4_migration = Migration::unapplied(
        "V4__create_user_token",
        &create_user_token::migration(ExecOpt::Up),
    )
    .unwrap();

    let v5_migration =
        Migration::unapplied("V5__create_role", &create_role::migration(ExecOpt::Up)).unwrap();

    let v6_migration = Migration::unapplied(
        "V6__create_user_role",
        &create_user_role::migration(ExecOpt::Up),
    )
    .unwrap();

    Runner::new(&[
        v1_migration,
        v2_migration,
        v3_migration,
        v4_migration,
        v5_migration,
        v6_migration,
    ])
}

pub fn get_rollback_migrations() -> Vec<RollbackScript> {
    vec![
        RollbackScript {
            name: "V1__create_user".to_string(),
            sql: create_user::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V2__create_auth".to_string(),
            sql: create_auth::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V3__create_token_type".to_string(),
            sql: create_token_type::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V4__create_user_token".to_string(),
            sql: create_user_token::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V5__create_role".to_string(),
            sql: create_role::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V6__create_user_role".to_string(),
            sql: create_user_role::migration(ExecOpt::Down),
        },
    ]
}
