pub mod api {
    pub mod gemini_client;
    pub mod youtube_chat;
}

pub mod media {
    pub mod audio;
    pub mod avatar_osc;
    pub mod tts_voicevox;
}

pub mod prompt;

pub use api::gemini_client::GeminiClient;
pub use api::youtube_chat;
pub use media::{audio, avatar_osc, tts_voicevox};
