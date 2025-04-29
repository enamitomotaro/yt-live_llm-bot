use crate::error::Result;
use anyhow::Context;
use async_stream::stream;
use tokio_stream::{Stream, wrappers::UnboundedReceiverStream};
use youtube_chat::live_chat::LiveChatClientBuilder;

pub async fn subscribe(url: &str) -> Result<impl Stream<Item = (String, String)>> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut client = LiveChatClientBuilder::new()
        .url(url)
        .unwrap()
        .on_chat(move |c| {
            let author = c.author.name.clone().unwrap_or_default();
            let text = c
                .message
                .iter()
                .filter_map(|m| match m {
                    youtube_chat::item::MessageItem::Text(t) => Some(t.as_str()),
                    _ => None,
                })
                .collect::<String>();
            let _ = tx.send((author, text));
        })
        .on_error(|e| tracing::warn!(?e, "ytchat error"))
        .build();

    client.start().await.context("start ytchat")?;

    tokio::spawn(async move {
        let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(3));
        loop {
            tick.tick().await;
            client.execute().await;
        }
    });

    Ok(stream! {
        for await (a,t) in UnboundedReceiverStream::new(rx) {
            if !a.is_empty() && !t.is_empty() { yield (a,t) }
        }
    })
}
