mod gemini;
use gemini::Gemini;

#[tokio::main]
async fn main() {
    // `Gemini::new("gemini-2.0-flash")` でクライアント生成
    let gemini = Gemini::new("gemini-2.0-flash");

    // `ask("こんにちは")` で 1 件目の回答テキストを取得
    match gemini.ask("こんにちは、調子はどう？").await {
        Ok(ans) => println!("Gemini の応答: {}", ans),
        Err(e) => eprintln!("Gemini API エラー: {}", e),
    }
}
