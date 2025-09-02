# LINE Bot RS - 開發代辦事項

## 🚀 階段一：專案基礎設定
- [x] 設定 Cargo.toml 依賴項
  - [x] 添加 tokio (異步運行時)
  - [x] 添加 axum (Web 框架)
  - [x] 添加 serde (JSON 序列化)
  - [x] 添加 reqwest (HTTP 客戶端)
  - [x] 添加 sha2 和 hmac (簽名驗證)
  - [x] 添加 dotenvy (環境變數管理)
  - [x] 添加 tracing (日誌記錄)

- [x] 建立專案目錄結構
  - [x] 建立 `src/lib.rs`
  - [x] 建立 `src/webhook/` 模組
  - [x] 建立 `src/line_api/` 模組
  - [x] 建立 `src/models/` 模組
  - [x] 建立 `src/handlers/` 模組
  - [x] 建立 `src/utils/` 模組

- [x] 環境配置
  - [x] 建立 `.env.example` 檔案
  - [x] 設定環境變數結構體
  - [x] 實作配置載入邏輯

## 🔧 階段二：核心功能開發

### Webhook 伺服器
- [x] 實作基礎 Axum 伺服器
- [x] 建立 `/webhook` POST 端點
- [x] 實作 LINE 簽名驗證中介軟體
- [x] 新增 CORS 支援
- [x] 實作錯誤處理中介軟體

### LINE API 資料模型
- [x] 定義 Webhook 事件結構體
  - [x] MessageEvent
  - [x] FollowEvent / UnfollowEvent
  - [x] JoinEvent / LeaveEvent
  - [x] PostbackEvent
  
- [x] 定義訊息類型結構體
  - [x] TextMessage
  - [x] StickerMessage
  - [x] ImageMessage
  - [x] TemplateMessage

- [x] 定義 LINE API 回應結構體
  - [x] ReplyMessage
  - [x] PushMessage
  - [x] MulticastMessage

### LINE API 客戶端
- [x] 實作 LINE API HTTP 客戶端
- [x] 實作 Reply API
- [x] 實作 Push API  
- [x] 實作 Multicast API
- [x] 實作 Profile API (取得用戶資訊)
- [x] 新增 API 錯誤處理

### 訊息處理邏輯
- [x] 建立訊息處理器 trait
- [x] 實作文字訊息處理器
- [x] 實作貼圖訊息處理器
- [x] 實作圖片訊息處理器
- [x] 實作事件路由系統

## 🧪 階段三：測試與品質保證
- [x] 撰寫單元測試
  - [x] 簽名驗證測試
  - [ ] 訊息處理測試
  - [x] LINE API 客戶端測試
  
- [x] 撰寫整合測試
  - [x] Webhook 端點測試
  - [x] 完整流程測試
  
- [x] 設定 CI/CD
  - [x] 新增 GitHub Actions
  - [x] 新增程式碼檢查 (clippy, fmt)
  - [x] 新增測試覆蓋率

## 📊 階段四：監控與日誌
- [x] 實作結構化日誌
  - [x] 請求/回應日誌
  - [x] 錯誤日誌
  - [x] 效能指標日誌
  
- [x] 新增健康檢查端點
- [ ] 實作指標收集 (可選)

## 🚀 階段五：部署與文件
- [x] 建立 Dockerfile
- [x] 建立 docker-compose.yml
- [x] 撰寫部署說明文件
- [x] 建立 API 文件
- [x] 撰寫使用範例

## 🔄 階段六：進階功能 (可選)
- [ ] Rich Menu 支援
- [ ] Flex Message 支援
- [ ] 檔案上傳功能
- [ ] 資料庫整合 (儲存用戶資料)
- [ ] 快取機制 (Redis)
- [ ] 訊息佇列支援
- [ ] 多語言支援

## 🔐 安全性檢查清單
- [ ] 輸入驗證
- [ ] SQL Injection 防護 (如有使用資料庫)
- [ ] XSS 防護
- [ ] 速率限制實作
- [ ] 敏感資料遮罩
- [ ] HTTPS 強制執行

## 📝 文件更新
- [x] 更新 README.md
- [x] API 文件
- [x] 部署指南
- [x] 故障排除指南
- [x] 貢獻指南

---

## 🎯 當前優先級
1. **高優先級**: 階段一、階段二核心功能
2. **中優先級**: 階段三測試、階段四監控
3. **低優先級**: 階段五部署、階段六進階功能

## 📋 完成標準
每個任務完成時應該：
- [ ] 程式碼通過所有測試
- [ ] 程式碼符合 Rust 慣例 (clippy 檢查通過)
- [ ] 程式碼格式化正確 (rustfmt)
- [ ] 相關文件已更新
- [ ] 功能經過手動測試驗證