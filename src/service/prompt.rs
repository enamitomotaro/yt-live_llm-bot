use crate::model::conversation::{Message, Role};
use crate::model::gemini as m;

pub const EMOTION_GUIDE: &str =
    "各文頭に [neutral|happy|sad|angry|relaxed|surprised] のタグを必ず付けて返答してください。";

/// コメントへの通常応答用プロンプト
pub fn build<'a>(
    system_prompt: &'a str,
    history: &'a [Message],
    max_history: usize,
) -> Vec<m::Content<'a>> {
    let mut contents = Vec::with_capacity(history.len() + 1);

    // システム指示（ガイド追加済み）
    let sys = format!("{system_prompt}\n{EMOTION_GUIDE}");
    contents.push(m::Content {
        role: "user",
        parts: vec![m::Part {
            text: Box::leak(sys.into_boxed_str()),
        }],
    });

    // 履歴
    let start = history.len().saturating_sub(max_history * 2);
    contents.extend(history[start..].iter().map(|msg| m::Content {
        role: match msg.role {
            Role::User => "user",
            Role::Bot => "model",
            Role::System => "system",
            Role::Assistant => "model", // Assuming Assistant should be treated like Bot
        },
        parts: vec![m::Part { text: &msg.text }],
    }));

    contents
}

/// コメントが途切れた際の自律トーク用プロンプト
pub fn build_spontaneous_prompt(spontaneous_prompt: &str) -> Vec<m::Content<'_>> {
    vec![m::Content {
        role: "user",
        parts: vec![m::Part {
            text: Box::leak(format!("{spontaneous_prompt}\n{EMOTION_GUIDE}").into_boxed_str()),
        }],
    }]
}
