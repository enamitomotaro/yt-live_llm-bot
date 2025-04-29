# yt-live_llm-bot

yt-live_llm-bot は **YouTube Live のチャット**を購読し  
→ **Google Gemini** が即座にコメントを生成  
→ **VOICEVOX** で音声合成  
→ **BlackHole** 仮想オーディオデバイスへ再生する、超軽量 CLI アプリです。

---

## ディレクトリ構成

```text
yt-live_llm-bot/
 └─ src
    ├─ bin/
    │   └─ main.rs          # ─ エントリポイント（Tokio runtime）
    │
    ├─ config.rs            # ─ .env 読み込み & Config 構造体
    ├─ error.rs             # ─ thiserror: crate 全域の共通 Error
    │
    ├─ model/               # ─ 外部 API DTO / 純粋データ
    │   └─ gemini.rs
    │
    ├─ service/             # ─ 外部 I/O & 副作用（“読む/書く”）
    │   ├─ gemini.rs        #   · Google Gemini HTTP クライアント
    │   ├─ youtube_chat.rs  #   · YouTube Live chat ストリーム
    │   ├─ tts.rs           #   · VOICEVOX エンジン呼び出し
    │   ├─ audio.rs         #   · BlackHole で WAV 再生
    │   └─ prompt.rs        #   · system_prompt + user_msg を合成
    │
    └─ lib.rs               # ─ mod 宣言 & 公開 API
```

---

## セットアップ

### 1. 依存ソフトをインストール（macOS 例）

```bash
# BlackHole 2ch（仮想オーディオデバイス）
brew install --cask blackhole-2ch

# VOICEVOX GUI でも OK / CLI 派は voicevox-engine を起動
brew install voicevox
# --- or ---
brew install voicevox-engine   # サーバは http://127.0.0.1:50021
```

### 2. `.env` と `prompt.txt` を用意

`.env` ファイル（プロジェクト直下）：

```env
# --- Gemini ------------------------------------
GEMINI_API_KEY=your_api_key_here
GEMINI_MODEL=gemini-2.0-flash

# --- VOICEVOX ----------------------------------
VOICEVOX_SPEAKER=47          # ナースロボ_タイプT - ノーマル

# --- YouTube -----------------------------------
YOUTUBE_LIVE_URL=https://www.youtube.com/watch?v=ec48fDQ1LKk

# --- System Prompt -----------------------------
BOT_SYSTEM_PROMPT_FILE=prompt.txt
```

`prompt.txt` は yt-live_llm-bot のキャラクターを決めるプロンプトです。例：

```
あなたは優しい VTuber AI です。語尾に「ですわ」を付けて話してください。
```

### 3. ビルド & 起動

```bash
cargo run --release -p yt-live_llm-bot
```

チャットに投稿されたメッセージ（先頭が `!` でないもの）に対して  
Gemini が応答を生成 → VOICEVOX が合成 → スピーカーから再生されます。

---

## VOICEVOX スピーカー ID 一覧

| ID  | キャラクター | スタイル |
| --- | ------------ | -------- |
| 0   | 四国めたん   | あまあま |
| 1   | ずんだもん   | あまあま |
| …   | …            | …        |
| 58  | 猫使ビィ     | ノーマル |

<details>
<summary>クリックでフルリストを表示</summary>

```
0  四国めたん       あまあま
1  ずんだもん       あまあま
2  四国めたん       ノーマル
3  ずんだもん       ノーマル
4  四国めたん       セクシー
5  ずんだもん       セクシー
6  四国めたん       ツンツン
7  ずんだもん       ツンツン
8  春日部つむぎ     ノーマル
9  波音リツ         ノーマル
10 雨晴はう         ノーマル
11 玄野武宏         ノーマル
12 白上虎太郎       ふつう
13 青山龍星         ノーマル
14 冥鳴ひまり       ノーマル
15 九州そら         あまあま
16 九州そら         ノーマル
17 九州そら         セクシー
18 九州そら         ツンツン
19 九州そら         ささやき
20 もち子さん       ノーマル
21 剣崎雌雄         ノーマル
22 ずんだもん       ささやき
23 WhiteCUL         ノーマル
24 WhiteCUL         たのしい
25 WhiteCUL         かなしい
26 WhiteCUL         びえーん
27 後鬼             人間ver.
28 後鬼             ぬいぐるみver.
29 No.7            ノーマル
30 No.7            アナウンス
31 No.7            読み聞かせ
32 白上虎太郎       わーい
33 白上虎太郎       びくびく
34 白上虎太郎       おこ
35 白上虎太郎       びえーん
36 四国めたん       ささやき
37 四国めたん       ヒソヒソ
38 ずんだもん       ヒソヒソ
39 玄野武宏         喜び
40 玄野武宏         ツンギレ
41 玄野武宏         悲しみ
42 ちび式じい       ノーマル
43 櫻歌ミコ         ノーマル
44 櫻歌ミコ         第二形態
45 櫻歌ミコ         ロリ
46 小夜/SAYO        ノーマル
47 ナースロボ＿タイプＴ ノーマル
48 ナースロボ＿タイプＴ 楽々
49 ナースロボ＿タイプＴ 恐怖
50 ナースロボ＿タイプＴ 内緒話
51 †聖騎士 紅桜†    ノーマル
52 雀松朱司         ノーマル
53 麒ヶ島宗麟       ノーマル
54 春歌ナナ         ノーマル
55 猫使アル         ノーマル
56 猫使アル         おちつき
57 猫使アル         うきうき
58 猫使ビィ         ノーマル
```

</details>

---

## カスタマイズ Tips

| やりたいこと                 | 変更箇所                           |
| ---------------------------- | ---------------------------------- |
| キャラクターの口調変更       | `prompt.txt`                       |
| デフォルトスピーカー変更     | `.env -> VOICEVOX_SPEAKER`         |
| Gemini モデル変更            | `.env -> GEMINI_MODEL`             |
| チャットのフィルタルール変更 | `service/youtube_chat.rs` の if 文 |

---

## 📜 ライセンス

MIT License  
© 2025 yt-live_llm-bot Project
