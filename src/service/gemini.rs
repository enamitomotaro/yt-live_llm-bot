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
        let base = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
        );
        let endpoint =
            Url::parse_with_params(&base, &[("key", api_key)]).context("construct endpoint url")?;

        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent(concat!("ai_tuber/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("build reqwest client")?;

        Ok(Self { client, endpoint })
    }

    pub async fn ask<'a>(&self, contents: &'a [m::Content<'a>]) -> Result<String> {
        self.client
            .post(self.endpoint.clone())
            .json(&m::GenerateReq {
                contents: contents.to_vec(),
            })
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
