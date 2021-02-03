use super::dtos::{Request, NodeResponse, Info};
use reqwest::{Client, Error, RequestBuilder, Response};
use serde_json::{json, to_value, Value};
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

    fn get_request_builder(&self) -> RequestBuilder {
        let url = self.url.clone();
        let request_builder = self.client.post(&url);

        request_builder.basic_auth(self.username.clone(), Some(self.pwd.clone()))
    }

    pub async fn get_wallet_info(&self) -> Result<NodeResponse<Info>, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("getwalletinfo"),
            None,
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<Info>>()
        .await
    }

    pub async fn get_addresses(&self) -> Result<NodeResponse<Vec<String>>, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("getaddressesbyaccount"),
            Some(json!("default")),
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<Vec<String>>>()
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
