use crate::error::Result;
use anyhow::Context;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

static HOST: &str = "http://127.0.0.1:50021";

pub async fn synth(text: &str, speaker: u16) -> Result<Vec<u8>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .context("build reqwest")?;

    let mut query: serde_json::Value = client
        .post(format!("{HOST}/audio_query"))
        .query(&[("text", text), ("speaker", &speaker.to_string())])
        .send()
        .await
        .context("audio_query")?
        .error_for_status()
        .context("audio_query non-2xx")?
        .json()
        .await
        .context("audio_query json")?;

    query["output_sampling_rate"] = json!(48_000);
    query["output_stereo"] = json!(true);

    let bytes = client
        .post(format!("{HOST}/synthesis"))
        .query(&[("speaker", &speaker.to_string())])
        .json(&query)
        .send()
        .await
        .context("synthesis")?
        .error_for_status()
        .context("synthesis non-2xx")?
        .bytes()
        .await
        .context("bytes")?;

    Ok(bytes.to_vec())
}
