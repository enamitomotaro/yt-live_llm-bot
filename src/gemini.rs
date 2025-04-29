use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// ---------- 送信用 JSON --------------------------------------------------

/// `/generateContent` のルート要素
#[derive(Serialize)]
struct GeminiRequest {
    /// メッセージ一覧（単発質問なので 1 件だけ入れる）
    contents: Vec<Content>,
}

/// ユーザーや AI の発話 1 件
#[derive(Serialize)]
struct Content {
    /// テキストなど複数パートをまとめた配列
    parts: Vec<Part>,
}

/// テキストパート
#[derive(Serialize)]
struct Part {
    text: String,
}

// ---------- 受信用 JSON --------------------------------------------------

/// 全体レスポンス
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

/// 候補 1 件
#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

/// 回答本文
#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

/// 回答のテキストパート
#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

// ---------- クライアント --------------------------------------------------

pub struct Gemini {
    client: Client,
    api_key: String,
    model: String,
}

impl Gemini {
    /// `GEMINI_API_KEY` 環境変数を読み込み、クライアントを生成
    pub fn new(model: &str) -> Self {
        let api_key = env::var("GEMINI_API_KEY")
            .expect("環境変数 GEMINI_API_KEY が未設定です");

        Gemini {
            client: Client::new(),
            api_key,
            model: model.to_string(),
        }
    }

    /// `prompt` を送信し、1 番目の候補テキストを返す
    /// 失敗時は `reqwest::Error` を返す
    /// 候補が 0 件なら空文字列を返す
    pub async fn ask(&self, prompt: &str) -> Result<String, reqwest::Error> {
        // 送信ボディを組み立てる
        let body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        // v1beta エンドポイント URL
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        // POST → JSON 受信
        let res: GeminiResponse = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        // 1 件目の候補があればそのテキスト、なければ空文字列
        Ok(res
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_default())
    }
}
