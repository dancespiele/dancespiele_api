use crate::guard::Roles;

pub fn set_user_role(role: Roles) -> String {
    match role {
        Roles::System => "admin".to_string(),
        Roles::Admin => "user".to_string(),
        _ => "customer".to_string(),
    }
}
