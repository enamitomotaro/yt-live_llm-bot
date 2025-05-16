//! Crate-wide error definitions.
//!
//! - すべての外部呼び出しには `anyhow::Context` で文脈を付与し、
//!   `tracing-error` レイヤでスタックトレースを一括出力するのが推奨。
//! - 追加のバリアントは “ドメイン固有/外部依存” のいずれかに揃えてください。

use thiserror::Error;

/// Convenience alias for functions that return the crate-level [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level error type for *ai_tuber*.
///
/// Preserve下位エラーは [`source`](std::error::Error::source) 経由でたどれるため、  
/// 上位層で `?` しても情報は失われません。
#[derive(Debug, Error)]
pub enum Error {
    // ───────────────────────────────
    // ドメイン固有
    // ───────────────────────────────
    /// 仮想ループバックデバイス **BlackHole** が見つからない。
    #[error("audio device \"BlackHole\" not found")]
    AudioDeviceNotFound,

    /// 設定値が不正。
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    /// 必須の環境変数が未設定。
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),

    /// Gemini API からのレスポンスが不正。
    #[error("invalid gemini response: {0}")]
    InvalidGeminiResponse(String),

    // ───────────────────────────────
    // 外部ライブラリ
    // ───────────────────────────────
    /// 任意の下位ライブラリからのエラーをそのまま保持。
    #[error(transparent)]
    External(#[from] anyhow::Error),
}
