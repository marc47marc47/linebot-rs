# LINE Bot RS - API 文件

## 概述

LINE Bot RS 提供以下 HTTP 端點：

- `/webhook` - 接收 LINE Platform 的 Webhook 事件
- `/health` - 健康檢查端點

## API 端點

### POST /webhook

接收來自 LINE Platform 的 Webhook 事件。

#### 請求標頭
- `Content-Type: application/json`
- `x-line-signature: sha256={signature}` - LINE 簽名驗證

#### 請求體
```json
{
  "destination": "Ub1234567890abcdef1234567890abcdef",
  "events": [
    {
      "type": "message",
      "replyToken": "replytoken123",
      "message": {
        "type": "text",
        "text": "Hello"
      },
      "timestamp": 1234567890,
      "source": {
        "type": "user",
        "userId": "Ub1234567890abcdef1234567890abcdef"
      },
      "mode": "active"
    }
  ]
}
```

#### 回應
- `200 OK` - 事件處理成功
- `400 Bad Request` - 請求格式錯誤或缺少必要標頭
- `401 Unauthorized` - 簽名驗證失敗

#### 支援的事件類型

##### 訊息事件 (Message Event)
用戶發送訊息時觸發。

**文字訊息**
```json
{
  "type": "message",
  "message": {
    "type": "text",
    "text": "使用者訊息內容"
  }
}
```

**貼圖訊息**
```json
{
  "type": "message",
  "message": {
    "type": "sticker",
    "packageId": "1",
    "stickerId": "1"
  }
}
```

**圖片訊息**
```json
{
  "type": "message",
  "message": {
    "type": "image",
    "contentProvider": {
      "type": "line"
    }
  }
}
```

##### 追蹤事件 (Follow Event)
用戶加為好友時觸發。

```json
{
  "type": "follow",
  "replyToken": "replytoken123",
  "source": {
    "type": "user",
    "userId": "Ub1234567890abcdef1234567890abcdef"
  }
}
```

##### 取消追蹤事件 (Unfollow Event)
用戶封鎖或刪除好友時觸發。

```json
{
  "type": "unfollow",
  "source": {
    "type": "user",
    "userId": "Ub1234567890abcdef1234567890abcdef"
  }
}
```

##### 加入群組事件 (Join Event)
機器人被加入群組時觸發。

```json
{
  "type": "join",
  "replyToken": "replytoken123",
  "source": {
    "type": "group",
    "groupId": "Cb1234567890abcdef1234567890abcdef"
  }
}
```

##### 離開群組事件 (Leave Event)
機器人被移出群組時觸發。

```json
{
  "type": "leave",
  "source": {
    "type": "group",
    "groupId": "Cb1234567890abcdef1234567890abcdef"
  }
}
```

##### Postback 事件
用戶點擊按鈕時觸發。

```json
{
  "type": "postback",
  "replyToken": "replytoken123",
  "postback": {
    "data": "action=button1"
  },
  "source": {
    "type": "user",
    "userId": "Ub1234567890abcdef1234567890abcdef"
  }
}
```

### GET /health

健康檢查端點，用於監控服務狀態。

#### 回應
- `200 OK` - 服務正常運行
- 回應體：`OK`

## 內建指令

Bot 支援以下文字指令：

| 指令 | 說明 | 範例回應 |
|------|------|----------|
| `hello`, `hi`, `你好`, `哈囉` | 打招呼 | "你好！有什麼可以幫助你的嗎？" |
| `help`, `幫助`, `說明` | 顯示幫助訊息 | 顯示可用指令列表 |
| `time`, `時間` | 顯示目前時間 | "目前時間：2024-01-01 12:00:00 UTC" |
| `sticker`, `貼圖` | 發送貼圖 | 發送預設貼圖 |
| `echo <訊息>`, `回音 <訊息>` | 回音功能 | "回音：<訊息>" |
| 其他文字 | 預設回應 | "我不太理解你的意思..." |

## 錯誤處理

### 簽名驗證錯誤
```
HTTP 401 Unauthorized
Body: Invalid signature
```

### 缺少必要標頭
```
HTTP 400 Bad Request  
Body: Missing signature header
```

### 請求體解析錯誤
```
HTTP 400 Bad Request
Body: Failed to read body
```

## LINE API 整合

Bot 內部使用以下 LINE API：

### Reply API
回覆用戶訊息。

```
POST https://api.line.me/v2/bot/message/reply
Authorization: Bearer {Channel Access Token}
Content-Type: application/json
```

### Push API  
主動發送訊息給用戶。

```
POST https://api.line.me/v2/bot/message/push
Authorization: Bearer {Channel Access Token}
Content-Type: application/json
```

### Multicast API
同時發送訊息給多位用戶。

```
POST https://api.line.me/v2/bot/message/multicast
Authorization: Bearer {Channel Access Token}
Content-Type: application/json
```

### Profile API
取得用戶個人資料。

```
GET https://api.line.me/v2/bot/profile/{userId}
Authorization: Bearer {Channel Access Token}
```

## 訊息類型

### 文字訊息
```json
{
  "type": "text",
  "text": "訊息內容"
}
```

### 貼圖訊息
```json
{
  "type": "sticker",
  "packageId": "1",
  "stickerId": "1"
}
```

### 模板訊息 (按鈕範本)
```json
{
  "type": "template",
  "altText": "按鈕模板",
  "template": {
    "type": "buttons",
    "text": "請選擇一個選項",
    "actions": [
      {
        "type": "message",
        "label": "選項 1",
        "text": "你選擇了選項 1"
      },
      {
        "type": "postback",
        "label": "選項 2", 
        "data": "action=option2"
      },
      {
        "type": "uri",
        "label": "網址",
        "uri": "https://example.com"
      }
    ]
  }
}
```

## 環境變數配置

| 變數名稱 | 必要 | 預設值 | 說明 |
|----------|------|--------|------|
| `CHANNEL_ACCESS_TOKEN` | ✅ | - | LINE Bot Channel Access Token |
| `CHANNEL_SECRET` | ✅ | - | LINE Bot Channel Secret |
| `PORT` | ❌ | `3000` | 伺服器監聽端口 |
| `HOST` | ❌ | `0.0.0.0` | 伺服器綁定地址 |
| `RUST_LOG` | ❌ | `info` | 日誌等級 |

## 安全考量

### 簽名驗證
所有 Webhook 請求都會進行 HMAC-SHA256 簽名驗證：

1. 使用 Channel Secret 作為密鑰
2. 對請求體進行 HMAC-SHA256 計算
3. 比對 `x-line-signature` 標頭中的簽名

### CORS 設定
預設允許所有來源的 CORS 請求。生產環境建議設定適當的 CORS 策略。

### 速率限制
建議在反向代理層實施速率限制，防止濫用。

## 監控

### 健康檢查
定期檢查 `/health` 端點確保服務正常。

### 日誌監控
設定 `RUST_LOG=info` 或更高等級監控重要事件：
- Webhook 事件接收
- LINE API 呼叫
- 錯誤和異常

### 指標收集
可整合 Prometheus 等監控工具收集：
- 請求處理時間
- API 呼叫成功率
- 錯誤率統計

---

更多詳細資訊請參考：
- [LINE Messaging API 文件](https://developers.line.biz/en/reference/messaging-api/)
- [專案 README](README.md)
- [部署指南](DEPLOYMENT.md)