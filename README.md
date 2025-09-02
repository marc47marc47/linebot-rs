# LINE Bot RS

使用 Rust 建立的 LINE Bot 接收跟發送訊息的 API 程式

## 功能特色

- ✅ 接收 LINE 平台的 Webhook 事件
- ✅ 支援多種訊息類型（文字、貼圖、圖片）
- ✅ LINE API 整合（Reply、Push、Multicast）
- ✅ 安全的簽名驗證
- ✅ 結構化日誌記錄
- ✅ 健康檢查端點

## 快速開始

### 1. 環境設定

複製環境變數範例檔案：
```bash
cp .env.example .env
```

編輯 `.env` 檔案，填入你的 LINE Bot 資訊：
```env
CHANNEL_ACCESS_TOKEN=your_channel_access_token_here
CHANNEL_SECRET=your_channel_secret_here
PORT=3000
HOST=0.0.0.0
RUST_LOG=info
```

### 2. 編譯與執行

```bash
# 編譯專案
cargo build --release

# 執行程式
cargo run
```

### 3. 設定 Webhook URL

在 LINE Developers Console 中設定 Webhook URL：
```
https://your-domain.com/webhook
```

## 支援的訊息指令

| 指令 | 說明 |
|------|------|
| `hello`, `hi`, `你好`, `哈囉` | 打招呼 |
| `help`, `幫助`, `說明` | 顯示幫助訊息 |
| `time`, `時間` | 顯示目前時間 |
| `sticker`, `貼圖` | 發送貼圖 |
| `echo <訊息>`, `回音 <訊息>` | 回音功能 |

## API 端點

### Webhook 接收端點
- **POST** `/webhook` - 接收 LINE 平台的事件通知

### 健康檢查端點
- **GET** `/health` - 伺服器健康狀態檢查

## 開發

### 執行測試
```bash
cargo test
```

### 檢查程式碼
```bash
cargo clippy
```

### 格式化程式碼
```bash
cargo fmt
```

## 專案架構

```
src/
├── main.rs              # 程式入口點
├── lib.rs               # 程式庫根模組
├── webhook/             # Webhook 處理
│   ├── mod.rs
│   ├── server.rs        # Axum 伺服器設定
│   └── handlers.rs      # Webhook 事件處理器
├── line_api/            # LINE API 客戶端
│   ├── mod.rs
│   └── client.rs        # HTTP 客戶端實作
├── models/              # 資料模型
│   ├── mod.rs
│   ├── events.rs        # LINE Webhook 事件結構
│   └── messages.rs      # 訊息類型定義
├── handlers/            # 訊息處理器
│   ├── mod.rs
│   └── message_handler.rs
└── utils/               # 工具函數
    ├── mod.rs
    ├── config.rs        # 配置管理
    └── signature.rs     # 簽名驗證
```

## 部署

### Docker 部署
```bash
# 建置 Docker 映像
docker build -t linebot-rs .

# 執行容器
docker run -p 3000:3000 --env-file .env linebot-rs
```

### 環境需求
- Rust 1.75+
- LINE Developers Console 帳號
- HTTPS 支援的網域（用於 Webhook）

## 貢獻

歡迎提交 Issue 和 Pull Request！

## 授權

本專案採用 MIT 授權條款。
