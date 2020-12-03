#[derive(Debug, SqlType, PartialEq, DbEnum, Serialize, Deserialize, AsExpression, Clone)]
#[DieselType = "Status_type"]
#[sql_type = "StatusType"]
#[postgres(type_name = "status_type")]
pub enum StatusType {
    Active,
    Inactive,
}
