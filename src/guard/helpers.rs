#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
        Roles::System => "SYSTEM".to_string(),
        Roles::Admin => "ADMIN".to_string(),
        Roles::User => "USER".to_string(),
        Roles::Customer => "CUSTOMER".to_string(),
    }
}

pub fn set_role(role: &str) -> Roles {
    match role {
        "SYSTEM" => Roles::System,
        "ADMIN" => Roles::Admin,
        "USER" => Roles::User,
        _ => Roles::Customer,
    }
}

pub fn _get_token_type(token_type: _TokenTypes) -> String {
    match token_type {
        _TokenTypes::SignUp => "signup".to_string(),
    }
}
