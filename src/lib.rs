//! crate root —— 層をまとめて re-export。main.rs からの依存面を簡潔に。

pub mod config;
pub mod error;

pub mod model {
    pub mod gemini;
}
pub mod service {
    pub mod audio;
    pub mod gemini;
    pub mod prompt;
    pub mod tts;
    pub mod youtube_chat;
}

pub use service::gemini::GeminiClient; // 新しい短縮名が必要ならここで再公開
