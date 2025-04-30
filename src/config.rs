use crate::error::{Error, Result};
use anyhow::Context;
use std::{env, fs};

fn must(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::MissingEnvVar(key.to_string()))
}

#[derive(Debug, Clone)]
pub struct Config {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub voicevox_speaker: u16,
    pub youtube_live_url: String,
    pub bot_system_prompt: String,
    pub max_history: usize,
    pub spontaneous_interval_sec: u64,
    pub spontaneous_prompt: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let bot_system_prompt = if let Ok(path) = env::var("BOT_SYSTEM_PROMPT_FILE") {
            fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?
        } else {
            env::var("BOT_SYSTEM_PROMPT")
                .unwrap_or_else(|_| "あなたは優しい VTuber AI です。".into())
        };

        // 自発的発話プロンプトの読み込み
        let spontaneous_prompt = if let Ok(path) = env::var("SPONTANEOUS_PROMPT_FILE") {
            fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?
        } else {
            "コメントが途切れたら自由に話してね。".into()
        };

        Ok(Self {
            gemini_api_key: must("GEMINI_API_KEY")?,
            gemini_model: env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.0-flash".into()),
            voicevox_speaker: env::var("VOICEVOX_SPEAKER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            youtube_live_url: must("YOUTUBE_LIVE_URL")?,
            bot_system_prompt,
            spontaneous_prompt,
            max_history: env::var("MAX_HISTORY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            spontaneous_interval_sec: env::var("SPONTANEOUS_INTERVAL_SEC") // 追加
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(180), // デフォルト3分間隔
        })
    }
}
