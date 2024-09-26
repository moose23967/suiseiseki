use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageJson {
    #[serde(rename = "chat_id")]
    pub chat_identifier: String,
    pub text: String,
    pub parse_mode: String,
}

pub struct TelegramApi {
    pub token: String,
    client: Client,
}

impl TelegramApi {
    pub fn new(token: String) -> TelegramApi {
        let client = Client::new();

        TelegramApi { token, client }
    }

    async fn request<T: Serialize>(&self, endpoint: String, json: T) {
        let token = &self.token;

        self.client
            .post(format!("https://api.telegram.org/bot{token}/{endpoint}"))
            .json(&json)
            .send()
            .await
            .unwrap();
    }

    pub async fn send_message(&self, send_message_json: SendMessageJson) {
        self.request(String::from("sendMessage"), send_message_json)
            .await;
    }
}
