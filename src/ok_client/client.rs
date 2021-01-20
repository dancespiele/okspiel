use super::dtos::{Request, WalletInfo};
use reqwest::{Client, Error, RequestBuilder, Response};
use serde_json::{json, to_value};
use std::sync::{Arc, Mutex};

pub struct RqClient {
    pub url: String,
    pub username: String,
    pub pwd: String,
    pub client: Client,
    pub nonce: Arc<Mutex<u64>>,
}

impl RqClient {
    pub fn new(url: String, username: String, pwd: String) -> Self {
        let client = Client::new();

        Self {
            url,
            username,
            pwd,
            client,
            nonce: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_request_builder(&self) -> RequestBuilder {
        let url = self.url.clone();
        let request_builder = self.client.post(&url);

        request_builder.basic_auth(self.username.clone(), Some(self.pwd.clone()))
    }

    pub async fn get_wallet_info(&self) -> Result<WalletInfo, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("getwalletinfo"),
            None,
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<WalletInfo>()
        .await
    }
}

#[tokio::test]
async fn should_get_wallet_info() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("prueba"),
        String::from("test"),
    );

    let wallet_info = rq_client.get_wallet_info().await.unwrap();

    println!("wallet info: {:?}", wallet_info);
}
