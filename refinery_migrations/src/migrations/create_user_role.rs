use super::helpers::{get_schema, ExecOpt};
use barrel::backend::Pg;
use barrel::{types, Migration};

pub fn migration(exec_opt: ExecOpt) -> String {
    let m = Migration::new();
    let mut schema = m.schema(get_schema());

    match exec_opt {
        ExecOpt::Up => {
            schema.create_table("user_role", |t| {
                t.add_column("id", types::text().unique(true).primary(true));
                t.add_column("user_id", types::text());
                t.add_column("role_id", types::text());
                t.inject_custom("created_at TIMESTAMP NOT NULL");
                t.inject_custom("updated_at TIMESTAMP NOT NULL");
                t.inject_custom("CONSTRAINT user_role_user_id_unique UNIQUE (user_id)");
                t.inject_custom(
                    "CONSTRAINT user_id_user_role_fkey foreign key (user_id) references dancespiele.user(id) ON DELETE CASCADE",
                );
                t.inject_custom(
                    "CONSTRAINT role_id_role_fkey foreign key (role_id) references dancespiele.role(id)",
                )
            });
            schema.make::<Pg>()
        }
        ExecOpt::Down => {
            schema.change_table("user_role", |t| {
                t.inject_custom("DROP CONSTRAINT user_role_user_id_unique");
                t.inject_custom("DROP CONSTRAINT user_id_user_role_fkey");
                t.inject_custom("DROP CONSTRAINT role_id_role_fkey");
            });
            schema.drop_table("user_role");
            schema.make::<Pg>()
        }
    }
}
