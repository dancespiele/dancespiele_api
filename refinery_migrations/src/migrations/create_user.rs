use super::helpers::{get_schema, ExecOpt};
use barrel::backend::Pg;
use barrel::{types, Migration};

pub fn migration(exec_opt: ExecOpt) -> String {
    let m = Migration::new();
    let mut schema = m.schema(get_schema());

    match exec_opt {
        ExecOpt::Up => {
            let mut status_type =
                "CREATE TYPE status_type AS ENUM ('active','inactive');".to_string();
            schema.create_table("user", |t| {
                t.add_column("id", types::text().unique(true).primary(true));
                t.add_column("email", types::text().unique(true));
                t.add_column("username", types::text().unique(true));
                t.inject_custom("status status_type NOT NULL");
                t.inject_custom("created_at TIMESTAMP NOT NULL");
                t.inject_custom("updated_at TIMESTAMP NOT NULL");
            });

            status_type.push_str(&schema.make::<Pg>());
            status_type
        }
        ExecOpt::Down => {
            let remove_status_type = "DROP TYPE status_type";
            schema.drop_table("user");
            let mut sql = schema.make::<Pg>();
            sql.push_str(remove_status_type);
            sql
        }
    }
}
