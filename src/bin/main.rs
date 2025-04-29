//! ① 入口：CLI から各 service を組み合わせるだけ
use ai_tuber::{
    config::Config,
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

    // ─ YouTube Live を購読 ─
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

    // ─ Read → Process → Write ─
    while let Some(user_msg) = rx.recv().await {
        let p = prompt::build(&cfg.bot_system_prompt, &user_msg);
        let rep = gemini.ask(&p).await?;
        let wav = tts::synth(&rep, cfg.voicevox_speaker).await?;
        audio::play(&wav)?;
    }
    Ok(())
}
