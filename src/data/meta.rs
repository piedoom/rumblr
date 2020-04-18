
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(rename = "msg")]
    message: String,
    status: usize,
}
