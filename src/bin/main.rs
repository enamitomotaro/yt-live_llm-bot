use ai_tuber::{
    config::Config,
    error::{Error, Result},
    model::{
        conversation::{Message, Role},
        emotion::Emotion,
    },
    service::{
        api::{gemini_client::GeminiClient, youtube_chat},
        media::{audio, avatar_osc, tts_voicevox},
        prompt,
    },
};

use regex::Regex;
use tokio_stream::StreamExt;

async fn parse_and_play(rep: &str, speaker: u16, tag_re: &Regex) -> Result<()> {
    let segments = tag_re
        .split(rep) // テキスト部分を列挙
        .zip(
            // 直前のタグとペアに
            std::iter::once("neutral") // 先頭の疑似タグ
                .chain(
                    tag_re
                        .find_iter(rep)
                        .map(|m| &rep[m.start() + 1..m.end() - 1]),
                ),
        );

    for (text, tag) in segments.filter(|(t, _)| !t.trim().is_empty()) {
        let emo = match tag {
            "happy" => Emotion::Happy,
            "sad" => Emotion::Sad,
            "angry" => Emotion::Angry,
            "relaxed" => Emotion::Relaxed,
            "surprised" => Emotion::Surprised,
            _ => Emotion::Neutral,
        };
        avatar_osc::set(emo)?;
        let wav = tts_voicevox::synth(text, speaker).await?;
        audio::play(&wav)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,ai_tuber=debug")
        .init();

    let cfg = Config::from_env()?;
    let gemini = GeminiClient::new(&cfg.gemini_api_key, &cfg.gemini_model)?;
    let mut history: Vec<Message> = Vec::new();
    let tag_re = Regex::new(r"(?i)\[(neutral|happy|sad|angry|relaxed|surprised)\]\s*")
        .map_err(|e| Error::External(e.into()))?;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);
    tokio::spawn({
        let url = cfg.youtube_live_url.clone();
        async move {
            if let Ok(stream) = youtube_chat::subscribe(&url).await {
                tokio::pin!(stream);
                while let Some((_, msg)) = stream.next().await {
                    if !msg.starts_with('!') {
                        let _ = tx.try_send(msg);
                    }
                }
            }
        }
    });

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
        cfg.spontaneous_interval.as_secs(),
    ));

    loop {
        tokio::select! {
            Some(user_msg) = rx.recv() => {
                history.push(Message { role: Role::User, text: std::borrow::Cow::Owned(user_msg.clone()) });

                if history.len() > cfg.max_history * 2 {
                    history.drain(0..history.len() - cfg.max_history * 2);
                }

                let req = prompt::build(&cfg.bot_system_prompt, &history, cfg.max_history);
                let rep = gemini.ask(&req).await?;
                history.push(Message { role: Role::Bot, text: std::borrow::Cow::Owned(rep.clone()) });

                parse_and_play(&rep, cfg.voicevox_speaker, &tag_re).await?;
            },

            _ = interval.tick() => {
                let req = prompt::build(&cfg.spontaneous_prompt, &[], 0);
                let rep = gemini.ask(&req).await?;
                history.push(Message { role: Role::Bot, text: std::borrow::Cow::Owned(rep.clone()) });

                parse_and_play(&rep, cfg.voicevox_speaker, &tag_re).await?;
            },
        }
    }
}
