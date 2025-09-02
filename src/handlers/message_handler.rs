use crate::models::{Event, OutgoingMessage};
use async_trait::async_trait;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_event(
        &self,
        event: Event,
    ) -> Result<Vec<OutgoingMessage>, Box<dyn std::error::Error>>;
}

pub struct DefaultMessageHandler;

#[async_trait]
impl MessageHandler for DefaultMessageHandler {
    async fn handle_event(
        &self,
        event: Event,
    ) -> Result<Vec<OutgoingMessage>, Box<dyn std::error::Error>> {
        match event {
            Event::Message(message_event) => match &message_event.message {
                crate::models::MessageType::Text { text } => {
                    Ok(vec![OutgoingMessage::text(format!("你說：{}", text))])
                }
                _ => Ok(vec![OutgoingMessage::text("收到訊息！")]),
            },
            Event::Follow(_) => Ok(vec![OutgoingMessage::text("歡迎關注！")]),
            _ => Ok(vec![]),
        }
    }
}
