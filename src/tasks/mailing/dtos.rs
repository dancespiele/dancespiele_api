#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NotifyEmail {
    pub pair: String,
    pub price: String,
    pub benefit: String,
    pub email: String,
}
