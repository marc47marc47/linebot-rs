# LINE Bot RS - 部署指南

## 部署選項概覽

本 LINE Bot 支援多種部署方式：

1. **Docker 部署**（推薦）
2. **Docker Compose 部署**
3. **雲端平台部署**
4. **本機部署**

## 前置需求

### LINE Bot 設定
1. 在 [LINE Developers Console](https://developers.line.biz/) 建立新的 Provider 和 Channel
2. 取得 Channel Access Token 和 Channel Secret
3. 設定 Webhook URL（部署完成後設定）

### 系統需求
- Docker 20.10+（推薦部署方式）
- Docker Compose 2.0+
- 或 Rust 1.75+（本機部署）

## 1. Docker 部署

### 基本部署

```bash
# 1. 建立環境變數檔案
cp .env.example .env

# 2. 編輯環境變數
# CHANNEL_ACCESS_TOKEN=your_channel_access_token
# CHANNEL_SECRET=your_channel_secret

# 3. 建置 Docker 映像
docker build -t linebot-rs .

# 4. 執行容器
docker run -d \
  --name linebot-rs \
  -p 3000:3000 \
  --env-file .env \
  linebot-rs
```

### 使用 Alpine 版本（更小體積）

```bash
docker build -f Dockerfile.alpine -t linebot-rs:alpine .
```

## 2. Docker Compose 部署

### 基本部署

```bash
# 1. 準備環境變數
cp .env.example .env
# 編輯 .env 檔案

# 2. 啟動服務
docker-compose up -d

# 3. 檢查狀態
docker-compose ps
docker-compose logs linebot
```

### 含反向代理部署

```bash
# 1. 複製 nginx 配置
cp nginx.conf.example nginx.conf
# 編輯 nginx.conf 設定你的網域

# 2. 準備 SSL 憑證到 ssl/ 目錄

# 3. 啟動含 nginx 的服務
docker-compose --profile with-proxy up -d
```

### 開發環境部署

```bash
# 啟動開發環境
docker-compose -f docker-compose.dev.yml up -d

# 含資料庫的開發環境
docker-compose -f docker-compose.dev.yml --profile with-db up -d
```

## 3. 雲端平台部署

### Heroku 部署

1. 安裝 Heroku CLI
2. 建立 Heroku 應用程式：

```bash
heroku create your-linebot-app
heroku config:set CHANNEL_ACCESS_TOKEN=your_token
heroku config:set CHANNEL_SECRET=your_secret
git push heroku main
```

### Railway 部署

1. 連結 GitHub 儲存庫到 Railway
2. 設定環境變數：
   - `CHANNEL_ACCESS_TOKEN`
   - `CHANNEL_SECRET`
3. Railway 會自動部署

### DigitalOcean App Platform

1. 建立 `app.yaml`：

```yaml
name: linebot-rs
services:
- name: web
  source_dir: /
  github:
    repo: your-username/linebot-rs
    branch: main
  run_command: ./target/release/linebot-rs
  environment_slug: rust
  instance_count: 1
  instance_size_slug: basic-xxs
  envs:
  - key: CHANNEL_ACCESS_TOKEN
    value: your_token
  - key: CHANNEL_SECRET
    value: your_secret
  - key: PORT
    value: "8080"
  http_port: 8080
```

### AWS ECS 部署

1. 建立 ECR 儲存庫
2. 推送 Docker 映像
3. 建立 ECS 任務定義
4. 設定 Application Load Balancer
5. 配置 Auto Scaling

## 4. 本機部署

### 開發環境

```bash
# 1. 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 設定環境變數
cp .env.example .env
# 編輯 .env

# 3. 執行程式
cargo run
```

### 生產環境

```bash
# 1. 建置 release 版本
cargo build --release

# 2. 執行
./target/release/linebot-rs
```

## 5. 反向代理設定

### Nginx 配置

使用提供的 `nginx.conf.example` 作為範本：

```bash
cp nginx.conf.example nginx.conf
```

重要設定：
- SSL 憑證路徑
- 速率限制
- 安全標頭
- 網域名稱

### Caddy 配置

建立 `Caddyfile`：

```caddyfile
your-domain.com {
    reverse_proxy linebot:3000
    
    rate_limit {
        zone webhook {
            key    {remote_host}
            events 10
            window 1s
        }
    }
    
    handle /webhook {
        rate_limit webhook
        reverse_proxy linebot:3000
    }
}
```

## 6. 監控與日誌

### 健康檢查

應用程式提供健康檢查端點：
```
GET /health
```

### 日誌監控

```bash
# Docker 日誌
docker logs -f linebot-rs

# Docker Compose 日誌
docker-compose logs -f linebot

# 系統日誌（systemd）
journalctl -u linebot-rs -f
```

### Prometheus 監控（可選）

如需進階監控，可以整合：
- Prometheus
- Grafana
- Alertmanager

## 7. 安全考量

### 環境變數安全
- 使用 Docker secrets 或 K8s secrets
- 不要在映像中包含敏感資料
- 使用環境變數注入

### 網路安全
- 僅暴露必要的端口
- 使用 HTTPS
- 實施速率限制
- 設定防火牆規則

### 容器安全
- 使用非 root 用戶
- 定期更新基礎映像
- 掃描映像漏洞

## 8. 故障排除

### 常見問題

1. **Webhook 驗證失敗**
   - 檢查 Channel Secret 是否正確
   - 確認請求體完整傳遞

2. **無法連接 LINE API**
   - 檢查 Channel Access Token
   - 確認網路連接

3. **端口衝突**
   - 修改 PORT 環境變數
   - 檢查防火牆設定

### 除錯模式

```bash
# 啟用詳細日誌
RUST_LOG=debug cargo run

# Docker 除錯
docker run -it --rm linebot-rs /bin/sh
```

## 9. 效能調優

### 容器資源限制

```yaml
services:
  linebot:
    deploy:
      resources:
        limits:
          memory: 128M
          cpus: '0.5'
        reservations:
          memory: 64M
          cpus: '0.25'
```

### 水平擴展

```bash
# Docker Compose 擴展
docker-compose up -d --scale linebot=3
```

## 10. 備份與恢復

### 配置備份
- 備份 `.env` 檔案
- 備份 SSL 憑證
- 備份應用程式配置

### 自動化部署

使用提供的 GitHub Actions 進行 CI/CD：
- 自動測試
- 自動建置 Docker 映像
- 自動部署到生產環境

---

如需更多協助，請參考：
- [GitHub Issues](https://github.com/your-repo/linebot-rs/issues)
- [LINE Bot 官方文件](https://developers.line.biz/en/docs/)