use super::{OpenApiSpec, ParsedSchema};
use anyhow::{anyhow, Result};
use reqwest::Client;

pub struct SchemaFetcher {
    url: String,
    anon_key: String,
    client: Client,
}

impl SchemaFetcher {
    pub fn new(url: String, anon_key: String) -> Result<Self> {
        Ok(Self {
            url: url.trim_end_matches('/').to_string(),
            anon_key,
            client: Client::new(),
        })
    }

    pub async fn fetch_schema(&self) -> Result<ParsedSchema> {
        let schema_url = format!("{}/rest/v1/", self.url);
        
        let response = self
            .client
            .get(&schema_url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.anon_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to fetch schema: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }

        let spec: OpenApiSpec = response.json().await?;
        ParsedSchema::from_openapi(spec)
    }
}