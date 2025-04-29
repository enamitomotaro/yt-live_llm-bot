//! Google Gemini 向け DTO

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GenerateReq<'a> {
    pub contents: [Content<'a>; 1],
}
#[derive(Serialize)]
pub struct Content<'a> {
    pub parts: [Part<'a>; 1],
}
#[derive(Serialize)]
pub struct Part<'a> {
    pub text: &'a str,
}
impl<'a> From<&'a str> for GenerateReq<'a> {
    fn from(p: &'a str) -> Self {
        Self {
            contents: [Content {
                parts: [Part { text: p }],
            }],
        }
    }
}

#[derive(Deserialize)]
pub struct GenerateRes {
    pub candidates: Vec<Candidate>,
}
#[derive(Deserialize)]
pub struct Candidate {
    pub content: RespContent,
}
#[derive(Deserialize)]
pub struct RespContent {
    pub parts: Vec<RespPart>,
}
#[derive(Deserialize)]
pub struct RespPart {
    pub text: String,
}

impl GenerateRes {
    pub fn first_text(self) -> String {
        self.candidates
            .into_iter()
            .flat_map(|c| c.content.parts)
            .map(|p| p.text)
            .next()
            .unwrap_or_default()
    }
}
