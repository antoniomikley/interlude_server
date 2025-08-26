use reqwest::{Client, IntoUrl};

pub async fn authorized_get_request(client: &Client, url: impl IntoUrl, token: &str) -> Result<String, reqwest::Error> {
        let response = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await?;

        Ok(response.text().await?)
}
