use reqwest::{Client, ClientBuilder, RequestBuilder, header};
use std::{fmt::Debug, str::FromStr};
use tokio::time;

#[derive(Clone)]
pub struct IntegrationBuilder(Integration);

impl IntegrationBuilder {
    pub fn build(mut self) -> anyhow::Result<Integration> {
        let client = ClientBuilder::new()
            .connect_timeout(time::Duration::from_secs(10))
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(header::USER_AGENT, "importer".parse()?);
                headers.insert(header::ACCEPT, "application/json".parse()?);
                if let Some(ref api_key) = self.0.api_key {
                    headers.insert(
                        header::HeaderName::from_str(&self.0.api_key_field)?,
                        header::HeaderValue::from_str(api_key)?,
                    );
                }
                headers
            })
            .build()?;

        self.0.client = Some(client);

        Ok(self.0)
    }
}

#[derive(Clone)]
pub struct Integration {
    name: String,
    url: String,
    api_key: Option<String>,
    api_key_field: String,
    client: Option<Client>,
}

impl Default for Integration {
    fn default() -> Self {
        Integration {
            name: "Yandex-Weather".to_string(),
            url: "https://api.weather.yandex.ru/v2/forecast".to_string(),
            api_key: None,
            api_key_field: "X-Api-Key".to_string(),
            client: None,
        }
    }
}

impl Debug for Integration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let api_key = self
            .api_key
            .as_ref()
            .map(|_| "*".repeat(8).to_string())
            .unwrap_or_default();
        f.debug_struct("Integration")
            .field("name", &self.name)
            .field("url", &self.url)
            .field("api_key", &api_key)
            .finish()
    }
}

impl Integration {
    pub fn from_env() -> IntegrationBuilder {
        IntegrationBuilder(Integration {
            name: std::env::var("API_NAME").unwrap_or_else(|_| "none".to_string()),
            url: std::env::var("API_URL").unwrap_or_else(|_| "none".to_string()),
            api_key: std::env::var("API_KEY").ok(),
            api_key_field: std::env::var("API_KEY_FIELD")
                .unwrap_or_else(|_| "X-Yandex-Weather-Key".to_string()),
            client: None,
        })
    }
    pub fn get_request(&self) -> RequestBuilder {
        self.client.clone().unwrap().get(&self.url)
    }
}
