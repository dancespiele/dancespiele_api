use crate::guard::Roles;

pub fn set_user_role(role: Roles) -> String {
    match role {
        Roles::System => "ADMIN".to_string(),
        Roles::Admin => "USER".to_string(),
        _ => "CUSTOMER".to_string(),
    }
}
