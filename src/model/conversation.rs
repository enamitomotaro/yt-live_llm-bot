//! Domain model: chat message & role.
//!
//! - `Role` は Gemini/OpenAI が使う `"system" | "user" | "assistant"` に合わせています。  
//! - `Message` は `Cow<'a, str>` で借用 or 所有を自動切替し、不要な `clone()` を回避。  
//! - `#[non_exhaustive]` で将来ロールが増えても後方互換を保証。  
//!
//! ## 例
//! ```rust
//! use ai_tuber::model::conversation::Message;
//!
//! let m = Message::user("こんにちは！");
//! assert_eq!(m.role.to_string(), "user");
//! ```

use std::{borrow::Cow, fmt};

use serde::{Deserialize, Serialize};

/// Speaker type in a conversation.
///
/// Variant names must stay in sync with Gemini/OpenAI API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Role {
    /// System-level instruction for the model.
    System,
    /// End-user message.
    User,
    /// Assistant (AI) response.
    Assistant,
    /// **Deprecated:** alias for [`Role::Assistant`] (kept for backward compatibility).
    #[serde(other)]
    Bot,
}

impl Role {
    /// Returns API-compatible string literal.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant | Role::Bot => "assistant",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Chat message.
///
/// Generic lifetime `'a` lets the text be either borrowed or owned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<'a> {
    pub role: Role,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
}

impl<'a> Message<'a> {
    /// Build a new message from `(role, text)`.
    pub fn new<T: Into<Cow<'a, str>>>(role: Role, text: T) -> Self {
        Self {
            role,
            text: text.into(),
        }
    }

    /* ----- Convenience helpers ----- */

    pub fn system<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Self::new(Role::System, text)
    }

    pub fn user<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Self::new(Role::User, text)
    }

    pub fn assistant<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Self::new(Role::Assistant, text)
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<(Role, T)> for Message<'a> {
    fn from((role, text): (Role, T)) -> Self {
        Self::new(role, text)
    }
}
