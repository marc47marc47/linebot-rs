# LINE Bot RS - 故障排除指南

## 常見問題與解決方案

### 1. 啟動問題

#### Q: 程式無法啟動，顯示環境變數錯誤

**錯誤訊息**：
```
Error: CHANNEL_ACCESS_TOKEN environment variable is required
```

**解決方案**：
1. 確認 `.env` 檔案存在並包含正確的環境變數
2. 檢查環境變數格式：
   ```env
   CHANNEL_ACCESS_TOKEN=your_actual_token_here
   CHANNEL_SECRET=your_actual_secret_here
   ```
3. 確認沒有額外的空格或引號

#### Q: 端口已被占用

**錯誤訊息**：
```
Error: Address already in use (os error 98)
```

**解決方案**：
1. 檢查端口使用狀況：
   ```bash
   netstat -tulpn | grep :3000
   lsof -i :3000
   ```
2. 停止占用端口的程序或更改端口：
   ```env
   PORT=3001
   ```

### 2. Webhook 問題

#### Q: LINE Platform 回報 Webhook 驗證失敗

**錯誤訊息**：LINE Developers Console 顯示 "Webhook 驗證失敗"

**解決方案**：
1. 確認 Channel Secret 設定正確
2. 檢查簽名驗證邏輯：
   ```bash
   # 查看詳細日誌
   RUST_LOG=debug cargo run
   ```
3. 確認 Webhook URL 正確設定（必須是 HTTPS）
4. 測試本地 Webhook：
   ```bash
   # 使用 ngrok 建立 HTTPS 隧道
   ngrok http 3000
   ```

#### Q: 收不到 Webhook 事件

**可能原因與解決方案**：

1. **Webhook URL 設定錯誤**
   - 檢查 LINE Developers Console 中的 Webhook URL
   - 確認 URL 以 `/webhook` 結尾
   - 確認使用 HTTPS

2. **防火牆問題**
   ```bash
   # 檢查防火牆狀態
   sudo ufw status
   
   # 開放端口
   sudo ufw allow 3000
   ```

3. **反向代理配置問題**
   - 檢查 nginx/caddy 配置
   - 確認代理正確轉發請求

### 3. LINE API 呼叫問題

#### Q: 回覆訊息失敗

**錯誤訊息**：
```
LINE API Error: Unauthorized
```

**解決方案**：
1. 檢查 Channel Access Token：
   - 確認 Token 正確且未過期
   - 重新生成 Channel Access Token
2. 檢查 API 請求格式：
   ```bash
   # 檢查詳細錯誤
   RUST_LOG=debug cargo run
   ```

#### Q: Push 訊息失敗

**錯誤訊息**：
```
LINE API Error: Bad Request
```

**解決方案**：
1. 確認用戶 ID 正確
2. 檢查訊息格式是否符合 LINE API 規範
3. 確認用戶已加好友（Push 訊息需要好友關係）

### 4. Docker 相關問題

#### Q: Docker 建置失敗

**錯誤訊息**：
```
error: could not compile `linebot-rs`
```

**解決方案**：
1. 檢查 Dockerfile 中的 Rust 版本
2. 清理 Docker 快取：
   ```bash
   docker system prune -a
   ```
3. 使用多階段建置避免快取問題：
   ```bash
   docker build --no-cache -t linebot-rs .
   ```

#### Q: 容器無法啟動

**錯誤訊息**：
```
standard_init_linux.go: exec: "./linebot-rs": no such file or directory
```

**解決方案**：
1. 檢查 Dockerfile 中的二進制文件路徑
2. 確認建置目標架構正確
3. 檢查檔案權限：
   ```dockerfile
   RUN chmod +x /app/linebot-rs
   ```

### 5. 效能問題

#### Q: 記憶體使用量過高

**症狀**：容器記憶體使用量持續增長

**解決方案**：
1. 監控記憶體使用：
   ```bash
   docker stats linebot-rs
   ```
2. 檢查是否有記憶體洩漏
3. 設定容器記憶體限制：
   ```yaml
   services:
     linebot:
       deploy:
         resources:
           limits:
             memory: 128M
   ```

#### Q: 回應時間過慢

**解決方案**：
1. 檢查網路連接
2. 優化 HTTP 客戶端設定
3. 實施快取機制
4. 使用連接池

### 6. 日誌與除錯

#### 啟用詳細日誌

```bash
# 開發環境
RUST_LOG=debug cargo run

# Docker 環境
docker run -e RUST_LOG=debug linebot-rs

# Docker Compose
environment:
  - RUST_LOG=debug
```

#### 查看日誌

```bash
# Docker 日誌
docker logs -f linebot-rs

# Docker Compose 日誌
docker-compose logs -f linebot

# 系統日誌
journalctl -u linebot-rs -f
```

### 7. 網路連接問題

#### Q: 無法連接到 LINE API

**錯誤訊息**：
```
Failed to send request: Connection timeout
```

**解決方案**：
1. 檢查網路連接：
   ```bash
   curl -I https://api.line.me/v2/bot/info
   ```
2. 檢查 DNS 解析
3. 檢查防火牆和代理設定
4. 增加請求超時時間

### 8. SSL/TLS 問題

#### Q: HTTPS 憑證問題

**錯誤訊息**：
```
SSL certificate verify failed
```

**解決方案**：
1. 更新 CA 憑證：
   ```bash
   # Debian/Ubuntu
   apt-get update && apt-get install ca-certificates
   
   # Alpine
   apk add --no-cache ca-certificates
   ```
2. 檢查系統時間是否正確
3. 檢查憑證有效期

### 9. 測試與驗證

#### 本地測試 Webhook

```bash
# 測試健康檢查
curl http://localhost:3000/health

# 模擬 LINE Webhook（需要正確的簽名）
curl -X POST http://localhost:3000/webhook \
  -H "Content-Type: application/json" \
  -H "x-line-signature: sha256=..." \
  -d '{"destination":"test","events":[]}'
```

#### 驗證配置

```bash
# 檢查環境變數
env | grep -E "(CHANNEL|PORT|HOST)"

# 檢查網路設定
netstat -tulpn | grep linebot

# 檢查程序狀態
ps aux | grep linebot
```

### 10. 部署後檢查清單

- [ ] 應用程式正常啟動
- [ ] 健康檢查端點回應 200
- [ ] Webhook URL 在 LINE Console 設定正確
- [ ] SSL 憑證有效
- [ ] 日誌正常輸出
- [ ] 測試基本訊息互動
- [ ] 檢查記憶體和 CPU 使用量
- [ ] 設定監控和告警

### 11. 尋求協助

如果以上解決方案都無法解決問題：

1. **查看詳細錯誤日誌**：啟用 `RUST_LOG=debug`
2. **檢查 GitHub Issues**：[專案 Issues 頁面](https://github.com/your-repo/linebot-rs/issues)
3. **建立新的 Issue**：包含以下資訊：
   - 錯誤訊息
   - 環境資訊（OS、Docker 版本等）
   - 重現步驟
   - 相關日誌

### 12. 預防措施

1. **定期備份**：配置文件、SSL 憑證
2. **監控設定**：記憶體、CPU、網路
3. **日誌輪轉**：避免日誌文件過大
4. **安全更新**：定期更新依賴項目
5. **測試環境**：在生產前測試更新

---

## 緊急聯絡資訊

- **技術支援**：[GitHub Issues](https://github.com/your-repo/linebot-rs/issues)
- **LINE 官方文件**：[LINE Developers](https://developers.line.biz/en/docs/)
- **社群討論**：[Discord/Slack 連結]