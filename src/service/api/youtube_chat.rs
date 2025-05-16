//! YouTube Live のチャットを `(author, message)` で非同期ストリーム化する。

use std::time::Duration;

use anyhow::Context;
use async_stream::stream;
use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    time::interval,
};
use tokio_stream::{Stream, wrappers::UnboundedReceiverStream};
use tracing::warn;
use youtube_chat::{item::MessageItem, live_chat::LiveChatClientBuilder};

use crate::error::Result;

/// ポーリング間隔（YouTube 制限を考慮して 3 秒）
const POLL_INTERVAL: Duration = Duration::from_secs(3);

/// 指定 URL のライブチャットを購読し、投稿 `(author, text)` を返す。
pub async fn subscribe(url: &str) -> Result<impl Stream<Item = (String, String)> + Send + 'static> {
    let (tx, rx) = unbounded_channel::<(String, String)>();

    /* ---------- LiveChatClient ---------- */
    let mut client = LiveChatClientBuilder::new()
        .url(url)
        .context("invalid YouTube live URL")?
        .on_chat(move |chat| forward_chat(&tx, chat))
        .on_error(|e| warn!(error = ?e, "YouTube chat error"))
        .build();

    client.start().await.context("start YouTube chat client")?;

    /* ---------- Poll loop ---------- */
    tokio::spawn(async move {
        let mut ticker = interval(POLL_INTERVAL);
        loop {
            ticker.tick().await;
            client.execute().await; // 返り値は ()
        }
    });

    /* ---------- Receiver stream ---------- */
    Ok(stream! {
        for await msg in UnboundedReceiverStream::new(rx) {
            yield msg;
        }
    })
}

/* --------------------------------------------------------------------- */
/*                             helpers                                   */
/* --------------------------------------------------------------------- */

/// `chat` から `(author, text)` を抽出して送信
fn forward_chat(tx: &UnboundedSender<(String, String)>, chat: youtube_chat::item::ChatItem) {
    let author = chat.author.name.clone().unwrap_or_default();

    let text = chat
        .message
        .iter()
        .filter_map(|m| match m {
            MessageItem::Text(t) => Some(t.as_str()),
            _ => None,
        })
        .collect::<String>();

    if !author.is_empty() && !text.is_empty() {
        let _ = tx.send((author, text));
    }
}
