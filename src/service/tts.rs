//! VoiceVox HTTP API でテキストを WAV バイト列に変換する。
//!
//! 1. `/audio_query` でクエリ JSON を取得  
//! 2. 必要なオプション (`output_sampling_rate`, `output_stereo`) を上書き  
//! 3. `/synthesis` へ POST し、WAV データ (Vec<u8>) を返す
//!
//! ## 依存クレート
//! ```toml
//! reqwest = { version = "0.11", features = ["json", "stream"] }
//! once_cell = "1"
//! anyhow    = "1"
//! ```
//!
//! **補足:** caller 側で再エンコードが不要なよう 48 kHz / ステレオ で出力します。

use std::time::Duration;

use anyhow::Context;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde_json::{Value, json};

use crate::error::Result;

/// VoiceVox サーバのベース URL
const HOST: &str = "http://127.0.0.1:50021";

/// API パス
mod endpoint {
    pub const AUDIO_QUERY: &str = "/audio_query";
    pub const SYNTHESIS: &str = "/synthesis";
}

/* --------------------------------------------------------------------- */
/*                           Global HTTP Client                          */
/* --------------------------------------------------------------------- */

static CLIENT: OnceCell<Client> = OnceCell::new();

#[inline]
fn client() -> Result<&'static Client> {
    Ok(CLIENT.get_or_try_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent(concat!("ai_tuber/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("build reqwest client")
    })?)
}

/// 指定テキストを合成し、WAV バイト列を返す。
///
/// * `speaker` – VoiceVox の話者 ID
pub async fn synth(text: &str, speaker: u16) -> Result<Vec<u8>> {
    let cl = client()?;

    /* ---------- 1. /audio_query ---------- */
    let mut query: Value = cl
        .post(format!("{HOST}{}", endpoint::AUDIO_QUERY))
        .query(&[("text", text), ("speaker", &speaker.to_string())])
        .send()
        .await
        .context("POST /audio_query")?
        .error_for_status()
        .context("/audio_query non-2xx")?
        .json()
        .await
        .context("deserialize audio_query")?;

    /* ---------- 2. オプション上書き ---------- */
    query["output_sampling_rate"] = json!(48_000);
    query["output_stereo"] = json!(true);

    /* ---------- 3. /synthesis ---------- */
    let bytes = cl
        .post(format!("{HOST}{}", endpoint::SYNTHESIS))
        .query(&[("speaker", &speaker.to_string())])
        .json(&query)
        .send()
        .await
        .context("POST /synthesis")?
        .error_for_status()
        .context("/synthesis non-2xx")?
        .bytes()
        .await
        .context("read wav bytes")?;

    // `Bytes` -> `Vec<u8>` にムーブ。clone() は発生しない。
    Ok(bytes.into())
}
