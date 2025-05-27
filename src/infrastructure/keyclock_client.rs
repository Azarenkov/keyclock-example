use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::config::Config;

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    refresh_token: String,
    id_token: String,
    expires_in: u32,
    refresh_expires_in: u32,
}

pub struct KeyclockClient {
    pub client: Client,
    pub config: Config,
}

impl KeyclockClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("Error create client"),
            config,
        }
    }

    pub async fn exchange_code_to_token(
        &self,
        code: &str,
    ) -> Result<TokenResponse, Box<dyn Error>> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.keyclock_base_url, self.config.keyclock_realm
        );

        let form_data = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.config.keyclock_client_id),
            ("client_secret", &self.config.client_secret),
            ("code", code),
            ("redirect_uri", &self.config.redirect_uri),
        ];

        let token_response = self.client.post(&token_url).form(&form_data).send().await;

        match token_response {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<TokenResponse>().await {
                        Ok(tokens) => Ok(tokens),
                        Err(e) => {
                            println!("Ошибка парсинга токенов: {}.", e);
                            Err(e.into())
                        }
                    }
                } else {
                    let error_text = response.text().await.unwrap_or_default();
                    println!("Ошибка получения токенов: {}", error_text);

                    Err("error_text".into())
                }
            }
            Err(e) => {
                println!("Ошибка запроса токенов: {}", e);
                Err(e.into())
            }
        }
    }
}
