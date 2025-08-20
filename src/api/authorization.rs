use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::ClientCredentials;

pub struct AccessToken {
    pub token: String,
    expiry: u64,
    auth_endpoint: String,
}

impl AccessToken {
    pub async fn new(
        client: &Client,
        credentials: &ClientCredentials,
        auth_endpoint: &str,
    ) -> anyhow::Result<Self> {
        #[derive(Serialize, Deserialize)]
        struct ResponseBody {
            access_token: String,
            expires_in: u64,
        }

        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Don't mess with the system clock. Use a reliable NTP server pls.")
            .as_secs();

        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");
        let response = client
            .post(auth_endpoint)
            .form(&form)
            .basic_auth(&credentials.client_id, Some(&credentials.client_secret))
            .send()
            .await?
            .text()
            .await?;
        let body: ResponseBody = serde_json::from_str(&response)?;

        Ok(Self {
            token: body.access_token,
            expiry: time_now + body.expires_in - 5,
            auth_endpoint: auth_endpoint.to_owned(),
        })
    }
    pub async fn refresh(
        &mut self,
        client: &Client,
        credentials: &ClientCredentials,
    ) -> anyhow::Result<()> {
        let refreshed_token = AccessToken::new(client, credentials, &self.auth_endpoint).await?;
        self.token = refreshed_token.token;
        self.expiry = refreshed_token.expiry;
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Don't mess with the system clock. Use a reliable NTP server pls.")
            .as_secs();

        if time_now < self.expiry {
            return false;
        }

        return true;
    }
}
