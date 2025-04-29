//! system_prompt + user_msg を 1 行で合成
pub fn build(system_prompt: &str, user_msg: &str) -> String {
    format!("{system_prompt}\nユーザー: {user_msg}\nアバター:")
}
