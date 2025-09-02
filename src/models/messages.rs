use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "sticker")]
    Sticker {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String,
    },
    #[serde(rename = "template")]
    Template {
        #[serde(rename = "altText")]
        alt_text: String,
        template: TemplateType,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateType {
    #[serde(rename = "buttons")]
    Buttons {
        text: String,
        actions: Vec<Action>,
        #[serde(rename = "thumbnailImageUrl", skip_serializing_if = "Option::is_none")]
        thumbnail_image_url: Option<String>,
        #[serde(rename = "imageAspectRatio", skip_serializing_if = "Option::is_none")]
        image_aspect_ratio: Option<String>,
        #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
        image_size: Option<String>,
        #[serde(
            rename = "imageBackgroundColor",
            skip_serializing_if = "Option::is_none"
        )]
        image_background_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "message")]
    Message { label: String, text: String },
    #[serde(rename = "postback")]
    Postback {
        label: String,
        data: String,
        #[serde(rename = "displayText", skip_serializing_if = "Option::is_none")]
        display_text: Option<String>,
    },
    #[serde(rename = "uri")]
    Uri { label: String, uri: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyMessageRequest {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub messages: Vec<OutgoingMessage>,
    #[serde(
        rename = "notificationDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushMessageRequest {
    pub to: String,
    pub messages: Vec<OutgoingMessage>,
    #[serde(
        rename = "notificationDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MulticastMessageRequest {
    pub to: Vec<String>,
    pub messages: Vec<OutgoingMessage>,
    #[serde(
        rename = "notificationDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_disabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub message: Option<String>,
    pub details: Option<Vec<ApiError>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub property: String,
}

impl OutgoingMessage {
    pub fn text<T: Into<String>>(text: T) -> Self {
        OutgoingMessage::Text { text: text.into() }
    }

    pub fn sticker<T: Into<String>>(package_id: T, sticker_id: T) -> Self {
        OutgoingMessage::Sticker {
            package_id: package_id.into(),
            sticker_id: sticker_id.into(),
        }
    }
}
