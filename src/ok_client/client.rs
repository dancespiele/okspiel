use super::dtos::{Info, NodeResponse, Request};
use core::f32;
use reqwest::{Client, Error, RequestBuilder};
use serde_json::{json, value::Value};
use std::sync::{Arc, Mutex};

pub struct RqClient {
    pub url: String,
    pub account: String,
    pub username: String,
    pub pwd: String,
    pub phrase: String,
    pub client: Client,
    pub nonce: Arc<Mutex<u64>>,
}

impl RqClient {
    pub fn new(
        url: String,
        account: String,
        username: String,
        pwd: String,
        phrase: String,
    ) -> Self {
        let client = Client::new();

        Self {
            url,
            account,
            username,
            pwd,
            phrase,
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
            Some(json!(vec![Value::from(self.account.clone())])),
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<Vec<String>>>()
        .await
    }

    pub async fn send_to_address(
        &self,
        to_address: String,
        amount: f64,
    ) -> Result<NodeResponse<String>, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("sendfrom"),
            Some(json!(vec![
                Value::from(self.account.clone()),
                Value::from(to_address),
                Value::from(amount)
            ])),
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<String>>()
        .await
    }

    pub async fn unlock_wallet(
        &self,
        time: u32,
        staking_mode: bool,
    ) -> Result<NodeResponse<Option<String>>, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("walletpassphrase"),
            Some(json!(vec![
                Value::from(self.phrase.clone()),
                Value::from(time),
                Value::from(staking_mode)
            ])),
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<Option<String>>>()
        .await
    }

    pub async fn lock_wallet(&self) -> Result<NodeResponse<Option<String>>, Error> {
        let rq = self.get_request_builder();

        rq.json(&Request::from((
            String::from("walletlock"),
            None,
            json!(*self.nonce),
        )))
        .send()
        .await?
        .json::<NodeResponse<Option<String>>>()
        .await
    }
}

#[tokio::test]
async fn should_get_wallet_info() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("default"),
        String::from("prueba"),
        String::from("test"),
        String::from("test"),
    );

    let wallet_info = rq_client.get_wallet_info().await.unwrap();

    println!("wallet info: {:?}", wallet_info);
}

#[tokio::test]
async fn should_get_addresses() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("default"),
        String::from("prueba"),
        String::from("test"),
        String::from("test"),
    );

    let addresses = rq_client.get_addresses().await.unwrap();

    println!("addresses: {:?}", addresses);
}

#[tokio::test]
async fn should_send_amount() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("default"),
        String::from("prueba"),
        String::from("test"),
        String::from("test"),
    );

    let response = rq_client
        .send_to_address("PMRhm1Zkt8fgBWjK6GKviXuTTr5ftEdQtx".to_string(), 0.1)
        .await;

    println!("response: {:?}", response);
}

#[tokio::test]
async fn should_unlock_wallet() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("default"),
        String::from("prueba"),
        String::from("test"),
        String::from("test"),
    );

    let response = rq_client.unlock_wallet(1000, false).await;

    println!("response: {:?}", response);
}

#[tokio::test]
async fn should_lock_wallet() {
    let rq_client = RqClient::new(
        String::from("http://127.0.0.1:6969/"),
        String::from("default"),
        String::from("prueba"),
        String::from("test"),
        String::from("test"),
    );

    let response = rq_client.lock_wallet().await;

    println!("response: {:?}", response);
}
