use crate::domain::conversation::{Message, Role};
use crate::model::gemini as m;

pub fn build_spontaneous_prompt(spontaneous_prompt: &str) -> Vec<m::Content<'_>> {
    vec![m::Content {
        role: "user",
        parts: vec![m::Part {
            text: spontaneous_prompt,
        }],
    }]
}

// 通常のプロンプト生成関数（問題なし）
pub fn build<'a>(
    system_prompt: &'a str,
    history: &'a [Message],
    max_history: usize,
) -> Vec<m::Content<'a>> {
    let mut contents = Vec::with_capacity(history.len() + 1);

    contents.push(m::Content {
        role: "user",
        parts: vec![m::Part {
            text: system_prompt,
        }],
    });

    let start = if history.len() > max_history * 2 {
        history.len() - max_history * 2
    } else {
        0
    };

    contents.extend(history[start..].iter().map(|msg| m::Content {
        role: match msg.role {
            Role::User => "user",
            Role::Bot => "model",
        },
        parts: vec![m::Part { text: &msg.text }],
    }));

    contents
}
