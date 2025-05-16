//! Runtime configuration loader.
//
//! - `.env` があれば優先して読み込む。
//! - パースエラーは [`Error::InvalidConfig`] で早期に失敗させる。
//! - インターバルは `std::time::Duration` で保持し、呼び出し側で即 `sleep` 可能。

use crate::error::{Error, Result};
use anyhow::Context;
use std::{env, fs, time::Duration};

/// デフォルト値集約
mod defaults {
    pub const GEMINI_MODEL: &str = "gemini-2.0-flash";
    pub const VOICEVOX_SPEAKER: u16 = 3;
    pub const MAX_HISTORY: usize = 10;
    /// 180 秒 = 3 分
    pub const SPONTANEOUS_INTERVAL_SEC: u64 = 180;
}

/// 必須の環境変数を取得する。存在しなければ [`Error::MissingEnvVar`].
fn env_must(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingEnvVar(key.to_string()))
}

/// 数値系をパースしつつ、失敗時は [`Error::InvalidConfig`].
fn parse_env<T>(key: &str, default: T) -> Result<T>
where
    T: std::str::FromStr + Clone,
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(key) {
        Ok(val) => val
            .parse::<T>()
            .with_context(|| format!("failed to parse {key}=\"{val}\""))
            .map_err(Error::from),
        Err(_) => Ok(default),
    }
}

/// アプリ全体で共有する設定。
#[derive(Debug, Clone)]
pub struct Config {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub voicevox_speaker: u16,
    pub youtube_live_url: String,
    pub bot_system_prompt: String,
    pub max_history: usize,
    pub spontaneous_interval: Duration,
    pub spontaneous_prompt: String,
}

impl Config {
    /// `.env` と OS 環境変数からロードする。
    pub fn from_env() -> Result<Self> {
        // .env は必須ではない
        dotenvy::dotenv().ok();

        let bot_system_prompt = read_text_or_env(
            "BOT_SYSTEM_PROMPT_FILE",
            "BOT_SYSTEM_PROMPT",
            "あなたは優しい VTuber AI です。",
        )?;

        let spontaneous_prompt = read_text_or_env(
            "SPONTANEOUS_PROMPT_FILE",
            "SPONTANEOUS_PROMPT", // 将来の拡張を見据えてキーを用意
            "コメントが途切れたら自由に話してね。",
        )?;

        Ok(Self {
            gemini_api_key: env_must("GEMINI_API_KEY")?,
            gemini_model: env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| defaults::GEMINI_MODEL.into()),
            voicevox_speaker: parse_env("VOICEVOX_SPEAKER", defaults::VOICEVOX_SPEAKER)?,
            youtube_live_url: env_must("YOUTUBE_LIVE_URL")?,
            bot_system_prompt,
            spontaneous_prompt,
            max_history: parse_env("MAX_HISTORY", defaults::MAX_HISTORY)?,
            spontaneous_interval: Duration::from_secs(parse_env(
                "SPONTANEOUS_INTERVAL_SEC",
                defaults::SPONTANEOUS_INTERVAL_SEC,
            )?),
        })
    }
}

/// 優先順位: 「*_FILE」→ 直接指定 → デフォルト
fn read_text_or_env(file_key: &str, direct_key: &str, fallback: &str) -> Result<String> {
    if let Ok(path) = env::var(file_key) {
        fs::read_to_string(&path)
            .with_context(|| format!("failed to read {path}"))
            .map_err(Error::from)
    } else {
        Ok(env::var(direct_key).unwrap_or_else(|_| fallback.into()))
    }
}
