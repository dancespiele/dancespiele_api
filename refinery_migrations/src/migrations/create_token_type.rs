use super::helpers::{get_schema, ExecOpt};
use barrel::backend::Pg;
use barrel::{types, Migration};

pub fn migration(exec_opt: ExecOpt) -> String {
    let m = Migration::new();
    let mut schema = m.schema(get_schema());

    match exec_opt {
        ExecOpt::Up => {
            schema.create_table("token_type", |t| {
                t.add_column("id", types::text().unique(true).primary(true));
                t.add_column("name", types::text().unique(true));
                t.inject_custom("created_at TIMESTAMP NOT NULL");
                t.inject_custom("updated_at TIMESTAMP NOT NULL");
            });
            schema.make::<Pg>()
        }
        ExecOpt::Down => {
            schema.drop_table("token_type");
            schema.make::<Pg>()
        }
    }
}
