# Line Bot RS - 系統開發文件

## 專案概述
使用 Rust 建立一個 LINE Bot 接收跟發送訊息的 API 程式

## 專案資訊
- **專案名稱**: linebot-rs
- **版本**: 0.1.0
- **Rust Edition**: 2024
- **開發語言**: Rust

## 系統架構

### 核心組件
1. **Webhook 伺服器**: 接收來自 LINE Platform 的 webhook 事件
2. **訊息處理器**: 處理接收到的訊息並生成回應
3. **LINE API 客戶端**: 發送訊息到 LINE Platform
4. **配置管理**: 管理 Channel Access Token 和 Channel Secret

### 技術棧
- **Web 框架**: 需要選擇 (建議 Axum 或 Warp)
- **HTTP 客戶端**: 需要選擇 (建議 Reqwest)
- **JSON 處理**: Serde
- **異步運行時**: Tokio
- **加密**: HMAC-SHA256 用於驗證 webhook

## API 設計

### Webhook 端點
- **路徑**: `/webhook`
- **方法**: POST
- **功能**: 接收 LINE Platform 的事件通知

### 核心功能
1. **接收訊息**
   - 文字訊息
   - 貼圖訊息
   - 圖片訊息
   - 其他媒體類型

2. **發送訊息**
   - Reply API: 回覆特定事件
   - Push API: 主動推送訊息
   - Multicast API: 群發訊息

3. **安全性**
   - Webhook 簽名驗證
   - HTTPS 支援
   - 環境變數配置

## 資料結構

### LINE Webhook 事件
- Message Event
- Follow/Unfollow Event
- Join/Leave Event
- Postback Event

### 回應格式
- Text Message
- Sticker Message
- Template Message
- Rich Menu

## 開發環境設定

### 必要工具
- Rust 1.75+
- Cargo
- LINE Developers Console 帳號

### 環境變數
```
CHANNEL_ACCESS_TOKEN=your_channel_access_token
CHANNEL_SECRET=your_channel_secret
PORT=3000
```

## 部署架構
- 本地開發: `cargo run`
- 容器化: Docker
- 雲端部署: 支援各種雲端平台

## 測試策略
- 單元測試: 核心邏輯測試
- 整合測試: API 端點測試
- 手動測試: LINE Bot 功能驗證

## 監控與日誌
- 結構化日誌記錄
- 錯誤追蹤
- 效能監控

## 安全考量
- 輸入驗證
- 速率限制
- 敏感資料保護
- CORS 設定

## 擴展性設計
- 模組化架構
- 插件系統支援
- 水平擴展能力