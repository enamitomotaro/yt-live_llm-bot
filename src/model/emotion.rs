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
    pub const fn clip(self) -> (&'static str, f32) {
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
