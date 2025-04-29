use thiserror::Error;

/// 本 crate 全域で使う失敗型
#[derive(Debug, Error)]
pub enum Error {
    // ────────────────────────────────────────
    // 独自分類
    // ────────────────────────────────────────
    #[error("audio device \"BlackHole\" not found")]
    BlackHoleNotFound,

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    // ────────────────────────────────────────
    // 外部ライブラリ由来はひとまとめ
    // ────────────────────────────────────────
    #[error(transparent)]
    External(#[from] anyhow::Error),
}

/// 薄い別名。Result<T> と書けて呼び出し側が楽になる
pub type Result<T, E = Error> = core::result::Result<T, E>;
