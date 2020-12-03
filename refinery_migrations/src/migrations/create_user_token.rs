use super::helpers::{get_schema, ExecOpt};
use barrel::backend::Pg;
use barrel::{types, Migration};

pub fn migration(exec_opt: ExecOpt) -> String {
    let m = Migration::new();
    let mut schema = m.schema(get_schema());

    match exec_opt {
        ExecOpt::Up => {
            schema.create_table("user_token", |t| {
                t.add_column("id", types::text().unique(true).primary(true));
                t.add_column("user_id", types::text());
                t.add_column("token_type_id", types::text());
                t.add_column("token", types::text());
                t.inject_custom("created_at TIMESTAMP NOT NULL");
                t.inject_custom("updated_at TIMESTAMP NOT NULL");
                t.inject_custom(
                    "CONSTRAINT user_id_user_token_fkey foreign key (user_id) references dancespiele.user(id) ON DELETE CASCADE",
                );
                t.inject_custom(
                    "CONSTRAINT token_type_id_user_token_fkey foreign key (token_type_id) references dancespiele.token_type(id) ON DELETE CASCADE",
                );
            });
            schema.make::<Pg>()
        }
        ExecOpt::Down => {
            schema.change_table("user_token", |t| {
                t.inject_custom("DROP CONSTRAINT user_id_user_token_fkey");
                t.inject_custom("DROP CONSTRAINT token_type_id_user_token_fkey");
            });
            schema.drop_table("user_token");
            schema.make::<Pg>()
        }
    }
}
