//! ① 入口：CLI から各 service を組み合わせるだけ
use ai_tuber::{
    config::Config,
    domain::conversation::{Message, Role},
    error::Result,
    service::{audio, gemini::GeminiClient, prompt, tts, youtube_chat},
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,ai_tuber=debug")
        .init();

    let cfg = Config::from_env()?;
    let gemini = GeminiClient::new(&cfg.gemini_api_key, &cfg.gemini_model)?;
    let mut history: Vec<Message> = Vec::new();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);
    tokio::spawn({
        let url = cfg.youtube_live_url.clone();
        let tx = tx.clone();
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
        cfg.spontaneous_interval_sec,
    ));

    loop {
        tokio::select! {
            Some(user_msg) = rx.recv() => {
                history.push(Message {
                    role: Role::User,
                    text: user_msg.clone(),
                });

                if history.len() > cfg.max_history * 2 {
                    let excess = history.len() - cfg.max_history * 2;
                    history.drain(0..excess);
                }

                let req_contents =
                    prompt::build(&cfg.bot_system_prompt, &history, cfg.max_history);

                let rep = gemini.ask(&req_contents).await?;

                history.push(Message {
                    role: Role::Bot,
                    text: rep.clone(),
                });

                let wav = tts::synth(&rep, cfg.voicevox_speaker).await?;
                audio::play(&wav)?;
            },
            _ = interval.tick() => { // タイマーが発動した時
                let req_contents =
                    prompt::build_spontaneous_prompt(&cfg.bot_system_prompt);

                let rep = gemini.ask(&req_contents).await?;

                history.push(Message {
                    role: Role::Bot,
                    text: rep.clone(),
                });

                let wav = tts::synth(&rep, cfg.voicevox_speaker).await?;
                audio::play(&wav)?;
            },
        }
    }
}
