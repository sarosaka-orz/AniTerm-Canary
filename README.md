# 🌸 AniTerm (v0.3.0)

> **更新代號： 「Tsumugi」 (紬)**
> 
> "在字節的經緯間，編織靈魂的絲線。"

AniTerm 是一個為開發者量身打造的 **動漫風格 AI 終端陪伴系統**。它不僅擁有精美的介面，更是一個居住在後台、能感知環境、擁有多重人格的虛擬生命。

![License](https://img.shields.io/badge/License-MIT-magenta.svg)
![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![AI](https://img.shields.io/badge/Powered%20by-Gemini%20AI-blue.svg)
![Shell](https://img.shields.io/badge/Shell-Zsh%20%7C%20Bash%20%7C%20Fish-green.svg)

---

## 🌟 v0.3.0 新特性：純粹靈魂的洗鍊

在 **Tsumugi** 版本中，我們優化了專案架構，正式移除所有 Web 相關的冗餘代碼，回歸到最純粹的 Rust 終端開發。這標誌著 AniTerm 進化為一個更加輕量、高效且專注於 CLI 體驗的虛擬工具。

### 🎭 目前可供召喚的人格：
*   **凜 (Rin)** - *原味傲嬌* 
    *   「哼，這點代碼也要我幫你改？別誤會了，我只是怕你把電腦燒掉而已！」
*   **澪 (Mio)** - *溫柔女僕*
    *   「主人辛苦了，要喝杯熱茶嗎？澪會永遠陪伴在側喔。」
*   **詩音 (Shion)** - *病嬌執念*
    *   「主人剛才是在看其他女孩的文件夾嗎？...呵呵，不准有別的东西喔。」

---

## ✨ 核心特性

- **💬 即時對話模式 (Chat Mode)**：透過 `aniterm --chat` 直接與當前人格進行深入對話，體驗 AI Studio 強大的生成能力。
- **🎭 多重人格系統**：自定義 display_name，支援運行時動態切換，fetch 介面自動適配人格。
- **🧠 雙模態回應系統**：
    - **本地模式**：針對常用指令提供極速的本地語料庫回應。
    - **AI 靈魂模式**：調用 Gemini API 生成深度的動態吐槽。
- **🌍 全方位環境感知**：監視 CPU、記憶體、時間、天氣、節日與音樂 (Kawaii Bass)。
- **🚀 Shell 深度整合**：支援 **Zsh**, **Bash**, 與 **Fish Shell**。
- **🔕 靜默守護**：當守護進程關閉時，Hooks 會自動進入靜默狀態，不會干擾您的正常作業。
- **⚙️ 靈活配置**：支援透過 `config.toml` 自定義全域 `system_prompt` 與模型名稱。

---

## 🛠️ 安裝與設定

### 1. 編譯專案
```bash
git clone https://github.com/sarosaka-orz/AniTerm-Canary.git
cd AniTerm-Canary
cargo build --release
```

### 2. 一鍵安裝 Hook
AniTerm 會自動偵測您的 Shell 並完成配置。

**Fish:** `echo "source ( $(pwd)/scripts/setup.sh | psub )" >> ~/.config/fish/config.fish`
**Zsh:** `echo "source <($(pwd)/scripts/setup.sh)" >> ~/.zshrc`
**Bash:** `echo "source <($(pwd)/scripts/setup.sh)" >> ~/.bashrc`

### 3. 配置 API Key
設定檔位於 `~/.config/aniterm/config.toml`。您可以直接填入 `api_key` 或設定環境變數 `GEMINI_API_KEY`。

---

## 🎮 使用方法

### 啟動守護進程
```bash
./target/release/anitermd
```

### 進入對話模式
```bash
aniterm --chat
```

### 多人格即時切換
```bash
aniterm --persona mio    # 切換到澪
aniterm --persona shion  # 切換到詩音
aniterm --persona rin    # 切換回凜
```

### 系統資訊獲取 (Fetch)
```bash
aniterm --fetch
```

---

## 🏗️ 技術架構

- **Backend**: Rust / Tokio (Async Runtime)
- **IPC**: Unix Domain Socket (`/tmp/aniterm.sock`)
- **AI**: Google Gemini API (支援自定義模型與 Prompt)
- **Display**: Unicode-Width 動態對齊與多色輸出

---

## ⚠️ 已知問題
- **Gemini API 延遲**: 由於雲端調用開銷，AI 模式可能會有 1-2 秒延遲。推薦在繁重任務（如編譯）時享受 AI 互動。

---

## 🏗️ 開發 Roadmap
- [x] 多人格切換系統
- [x] Unicode 視覺寬度對齊
- [x] **Chat 互動模式** (實作 REPL 長談功能)
- [x] **架構純粹化** (移除所有 Web 遺留代碼)
- [ ] **雲端語料庫同步** (Auto-Update Registry)
- [ ] **視覺化 ASCII Art 大改版**

---

**"看什麼看！還不快去幫我點個 Star！笨蛋主人！"** 🌸

