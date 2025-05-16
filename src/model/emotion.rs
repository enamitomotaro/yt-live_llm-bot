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
    /// Unity-VMC の BlendShape 名と値を返す。
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

impl std::str::FromStr for Emotion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_ascii_lowercase().as_str() {
            "neutral" => Ok(Self::Neutral),
            "happy" => Ok(Self::Happy),
            "sad" => Ok(Self::Sad),
            "angry" => Ok(Self::Angry),
            "relaxed" => Ok(Self::Relaxed),
            "surprised" => Ok(Self::Surprised),
            _ => Err(()),
        }
    }
}
