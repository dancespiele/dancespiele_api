pub enum Roles {
    System,
    Admin,
    User,
    Customer,
}

pub enum _TokenTypes {
    SignUp,
}

pub fn get_role(role: Roles) -> String {
    match role {
        Roles::System => "system".to_string(),
        Roles::Admin => "admin".to_string(),
        Roles::User => "user".to_string(),
        Roles::Customer => "customer".to_string(),
    }
}

pub fn set_role(role: &str) -> Roles {
    match role {
        "system" => Roles::System,
        "admin" => Roles::Admin,
        "user" => Roles::User,
        _ => Roles::Customer,
    }
}

pub fn _get_token_type(token_type: _TokenTypes) -> String {
    match token_type {
        _TokenTypes::SignUp => "signup".to_string(),
    }
}
