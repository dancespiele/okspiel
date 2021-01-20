use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub params: Option<Value>,
    pub id: Value,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    walletversion: f32,
    balance: f32,
    txcount: f32,
    keypoololdest: f32,
    keypoolsize: f32,
    unlocked_until: f32,
}

#[derive(Debug, Deserialize)]
pub struct WalletInfo {
    result: Info,
}

impl From<(String, Option<Value>, Value)> for Request {
    fn from(rq_options: (String, Option<Value>, Value)) -> Self {
        let (method, params, id) = rq_options;

        Self { method, params, id }
    }
}
