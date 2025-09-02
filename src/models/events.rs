use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookRequest {
    pub destination: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "message")]
    Message(MessageEvent),
    #[serde(rename = "follow")]
    Follow(FollowEvent),
    #[serde(rename = "unfollow")]
    Unfollow(UnfollowEvent),
    #[serde(rename = "join")]
    Join(JoinEvent),
    #[serde(rename = "leave")]
    Leave(LeaveEvent),
    #[serde(rename = "postback")]
    Postback(PostbackEvent),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    pub reply_token: String,
    pub message: MessageType,
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowEvent {
    pub reply_token: String,
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnfollowEvent {
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinEvent {
    pub reply_token: String,
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LeaveEvent {
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostbackEvent {
    pub reply_token: String,
    pub postback: PostbackData,
    pub timestamp: u64,
    pub source: Source,
    pub mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostbackData {
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessageType {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "sticker")]
    Sticker {
        #[serde(rename = "stickerId")]
        sticker_id: String,
        #[serde(rename = "packageId")]
        package_id: String,
    },
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "contentProvider")]
        content_provider: ContentProvider,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentProvider {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "external")]
    External {
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Source {
    #[serde(rename = "user")]
    User {
        #[serde(rename = "userId")]
        user_id: String,
    },
    #[serde(rename = "group")]
    Group {
        #[serde(rename = "groupId")]
        group_id: String,
        #[serde(rename = "userId")]
        user_id: Option<String>,
    },
    #[serde(rename = "room")]
    Room {
        #[serde(rename = "roomId")]
        room_id: String,
        #[serde(rename = "userId")]
        user_id: Option<String>,
    },
}
