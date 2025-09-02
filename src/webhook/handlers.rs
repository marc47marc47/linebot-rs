use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::models::{Event, MessageEvent, MessageType, OutgoingMessage, WebhookRequest};
use crate::utils::{ReplyTokenValidator, SensitiveDataMasker, TextValidator, record_webhook_event};
use crate::webhook::server::AppState;

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WebhookRequest>,
) -> impl IntoResponse {
    info!("Received webhook with {} events", payload.events.len());

    for event in payload.events {
        if let Err(e) = process_event(&state, event).await {
            error!("Failed to process event: {}", e);
        }
    }

    StatusCode::OK
}

async fn process_event(state: &AppState, event: Event) -> Result<(), Box<dyn std::error::Error>> {
    // 記錄 webhook 事件指標
    let event_type = match &event {
        Event::Message(_) => "message",
        Event::Follow(_) => "follow",
        Event::Unfollow(_) => "unfollow",
        Event::Join(_) => "join",
        Event::Leave(_) => "leave",
        Event::Postback(_) => "postback",
    };
    record_webhook_event(event_type);

    match event {
        Event::Message(message_event) => {
            handle_message_event(state, message_event).await?;
        }
        Event::Follow(follow_event) => {
            info!("User followed: {:?}", follow_event);
            let welcome_message = OutgoingMessage::text("歡迎使用 LINE Bot！");
            state
                .line_client
                .reply_message(&follow_event.reply_token, vec![welcome_message])
                .await?;
        }
        Event::Unfollow(unfollow_event) => {
            info!("User unfollowed: {:?}", unfollow_event);
        }
        Event::Join(join_event) => {
            info!("Bot joined: {:?}", join_event);
            let welcome_message = OutgoingMessage::text("大家好！我是你們的 LINE Bot 助手！");
            state
                .line_client
                .reply_message(&join_event.reply_token, vec![welcome_message])
                .await?;
        }
        Event::Leave(leave_event) => {
            info!("Bot left: {:?}", leave_event);
        }
        Event::Postback(postback_event) => {
            info!("Postback received: {:?}", postback_event);
            let response =
                OutgoingMessage::text(format!("收到 postback: {}", postback_event.postback.data));
            state
                .line_client
                .reply_message(&postback_event.reply_token, vec![response])
                .await?;
        }
    }

    Ok(())
}

async fn handle_message_event(
    state: &AppState,
    event: MessageEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    // 驗證 reply token
    if let Err(validation_error) = ReplyTokenValidator::validate(&event.reply_token) {
        warn!("Invalid reply token: {}", validation_error);
        return Err(format!("Invalid reply token: {}", validation_error).into());
    }

    // 記錄敏感資料（遮罩處理）
    info!(
        "Processing message from user: {}",
        SensitiveDataMasker::mask_user_id(&get_user_id_from_source(&event.source))
    );

    let text_validator = TextValidator::new().max_length(1000);
    let response_messages = match &event.message {
        MessageType::Text { text } => {
            // 驗證文字輸入
            if let Err(validation_error) = text_validator.validate(text) {
                warn!("Invalid text input: {}", validation_error);
                vec![OutgoingMessage::text("抱歉，您的訊息包含無效內容。")]
            } else {
                info!("Received text message: {}", text);
                handle_text_message(text)
            }
        }
        MessageType::Sticker {
            sticker_id,
            package_id,
        } => {
            info!(
                "Received sticker: package_id={}, sticker_id={}",
                package_id, sticker_id
            );
            vec![OutgoingMessage::text("收到貼圖！")]
        }
        MessageType::Image { .. } => {
            info!("Received image message");
            vec![OutgoingMessage::text("收到圖片！")]
        }
    };

    if !response_messages.is_empty() {
        state
            .line_client
            .reply_message(&event.reply_token, response_messages)
            .await?;
    }

    Ok(())
}

fn handle_text_message(text: &str) -> Vec<OutgoingMessage> {
    match text.to_lowercase().trim() {
        "hello" | "hi" | "你好" | "哈囉" => {
            vec![OutgoingMessage::text("你好！有什麼可以幫助你的嗎？")]
        }
        "help" | "幫助" | "說明" => {
            vec![OutgoingMessage::text(
                "可用指令：\n• hello - 打招呼\n• help - 顯示說明\n• time - 顯示目前時間\n• sticker - 發送貼圖",
            )]
        }
        "time" | "時間" => {
            let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            vec![OutgoingMessage::text(format!("目前時間：{}", now))]
        }
        "sticker" | "貼圖" => {
            vec![OutgoingMessage::sticker("1", "1")]
        }
        _ => {
            if let Some(echo_text) = text.strip_prefix("echo ") {
                vec![OutgoingMessage::text(format!("回音：{}", echo_text))]
            } else if let Some(echo_text) = text.strip_prefix("回音 ") {
                vec![OutgoingMessage::text(format!("回音：{}", echo_text))]
            } else {
                vec![OutgoingMessage::text(
                    "我不太理解你的意思，試試輸入 'help' 查看可用指令。",
                )]
            }
        }
    }
}

fn get_user_id_from_source(source: &crate::models::Source) -> String {
    match source {
        crate::models::Source::User { user_id } => user_id.clone(),
        crate::models::Source::Group {
            user_id: Some(user_id),
            ..
        } => user_id.clone(),
        crate::models::Source::Room {
            user_id: Some(user_id),
            ..
        } => user_id.clone(),
        _ => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_text_message_hello() {
        let result = handle_text_message("hello");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert_eq!(text, "你好！有什麼可以幫助你的嗎？");
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_help() {
        let result = handle_text_message("help");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert!(text.contains("可用指令"));
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_time() {
        let result = handle_text_message("time");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert!(text.contains("目前時間"));
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_sticker() {
        let result = handle_text_message("sticker");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Sticker {
            package_id,
            sticker_id,
        } = &result[0]
        {
            assert_eq!(package_id, "1");
            assert_eq!(sticker_id, "1");
        } else {
            panic!("Expected sticker message");
        }
    }

    #[test]
    fn test_handle_text_message_echo() {
        let result = handle_text_message("echo test message");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert_eq!(text, "回音：test message");
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_echo_chinese() {
        let result = handle_text_message("回音 測試訊息");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert_eq!(text, "回音：測試訊息");
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_unknown() {
        let result = handle_text_message("unknown command");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert!(text.contains("我不太理解你的意思"));
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_handle_text_message_case_insensitive() {
        let result = handle_text_message("HELLO");
        assert_eq!(result.len(), 1);
        if let OutgoingMessage::Text { text } = &result[0] {
            assert_eq!(text, "你好！有什麼可以幫助你的嗎？");
        } else {
            panic!("Expected text message");
        }
    }
}
