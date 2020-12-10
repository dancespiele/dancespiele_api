#[derive(Debug, SqlType, PartialEq, DbEnum, Serialize, Deserialize, AsExpression, Clone)]
#[DieselType = "Status_type"]
#[sql_type = "StatusType"]
#[postgres(type_name = "status_type")]
pub enum StatusType {
    Active,
    Inactive,
}

pub fn get_status_type(status: &str) -> StatusType {
    match status {
        "ACTIVE" => StatusType::Active,
        "INACTIVE" => StatusType::Inactive,
        &_ => StatusType::Active,
    }
}
