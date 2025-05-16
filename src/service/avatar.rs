//! OSC で Unity‑VMC の BlendShape を操作する。
//!
//! ```text
//! 1. 前回の表情を 0.0 にする
//! 2. 今回の表情を 1.0 にする
//! 3. Apply を送る
//! ```
//! 送信先は `127.0.0.1:39539` 固定。

use std::{net::UdpSocket, sync::Mutex};

use anyhow::Context;
use once_cell::sync::{Lazy, OnceCell};
use rosc::{OscMessage, OscPacket, OscType, encoder};

use crate::error::{Error, Result};

/// 対応している表情セット。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Relaxed,
    Surprised,
}

impl Emotion {
    /// OSC ブレンド名と値を返す。
    const fn clip(self) -> (&'static str, f32) {
        match self {
            Self::Neutral => ("Neutral", 0.0),
            Self::Happy => ("Joy", 1.0),
            Self::Sad => ("Sorrow", 1.0),
            Self::Angry => ("Angry", 1.0),
            Self::Relaxed => ("Relaxed", 1.0),
            Self::Surprised => ("Fun", 1.0),
        }
    }
}

/* ───────────────────── グローバル状態 ───────────────────── */

/// 送信用 UDP ソケット（初回だけ bind）。
static SOCKET: OnceCell<UdpSocket> = OnceCell::new();

/// 前回適用した表情名。
static LAST: Lazy<Mutex<Option<&'static str>>> = Lazy::new(|| Mutex::new(None));

/* ───────────────────── 内部ユーティリティ ───────────────────── */

/// ソケットを取得する。`OnceCell` により 1 度だけ初期化される。
fn socket() -> Result<&'static UdpSocket> {
    SOCKET.get_or_try_init(|| {
        UdpSocket::bind("0.0.0.0:0")
            .context("bind UDP socket")
            .map_err(Error::from)
    })
}

/// OSC `/VMC/Ext/Blend/Val` を送信。
fn send_blend_val(sock: &UdpSocket, name: &str, value: f32) -> Result<()> {
    let msg = OscMessage {
        addr: "/VMC/Ext/Blend/Val".into(),
        args: vec![OscType::String(name.into()), OscType::Float(value)],
    };
    let buf = encoder::encode(&OscPacket::Message(msg)).context("encode OSC packet")?;
    sock.send_to(&buf, "127.0.0.1:39539")
        .context("send OSC packet")?;
    Ok(())
}

/// `/VMC/Ext/Blend/Apply` を送信。
fn send_apply(sock: &UdpSocket) -> Result<()> {
    let msg = OscMessage {
        addr: "/VMC/Ext/Blend/Apply".into(),
        args: vec![],
    };
    let buf = encoder::encode(&OscPacket::Message(msg)).context("encode OSC packet")?;
    sock.send_to(&buf, "127.0.0.1:39539")
        .context("send OSC packet")?;
    Ok(())
}

/* ───────────────────── 公開 API ───────────────────── */

/// 表情を切り替える。
///
/// 同じ表情を連続で指定しても――
/// - **Neutral** は毎回 0.0 を送るため効果がある
/// - それ以外は無駄な OSC を抑制するためスキップ
pub fn set(em: Emotion) -> Result<()> {
    let (name, val) = em.clip();
    let sock = socket()?;

    // 前回の表情を 0.0 に戻す
    {
        let mut last = LAST.lock().unwrap(); // Poison 化しない想定
        if let Some(prev) = *last {
            if prev != name || val == 0.0 {
                send_blend_val(sock, prev, 0.0)?;
            }
        }

        // 今回の表情を適用
        if val > 0.0 {
            send_blend_val(sock, name, val)?;
        }
        send_apply(sock)?;
        *last = Some(name);
    }

    Ok(())
}
