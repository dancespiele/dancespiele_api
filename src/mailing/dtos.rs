#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StopLossSet {
    pub pair: String,
    pub price: String,
    pub benefit: String,
}
