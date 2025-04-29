use crate::{error::Result, model::gemini as m};
use anyhow::Context;
use reqwest::{Client, Url};
use std::time::Duration;

#[derive(Clone)]
pub struct GeminiClient {
    client: Client,
    endpoint: Url,
}

impl GeminiClient {
    pub fn new(api_key: &str, model: &str) -> Result<Self> {
        let endpoint = Url::parse("https://generativelanguage.googleapis.com")
            .context("parse base url")?
            .join(&format!("/v1beta/models/{model}:generateContent"))
            .context("join model path")?
            .join(&format!("?key={api_key}"))
            .context("append api key")?;

        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent(concat!("ai_tuber/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("build reqwest client")?;

        Ok(Self { client, endpoint })
    }

    pub async fn ask(&self, prompt: &str) -> Result<String> {
        self.client
            .post(self.endpoint.clone())
            .json(&m::GenerateReq::from(prompt))
            .send()
            .await
            .context("POST gemini")?
            .error_for_status()
            .context("non-2xx")?
            .json::<m::GenerateRes>()
            .await
            .context("parse json")
            .map(|r| r.first_text())
            .map_err(Into::into)
    }
}
