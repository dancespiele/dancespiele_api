#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Percentage {
    pub pair: String,
    pub new_stop_loss: String,
    pub next_stop_loss: String,
}
