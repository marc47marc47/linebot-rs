use crate::models::{
    ApiResponse, MulticastMessageRequest, OutgoingMessage, PushMessageRequest, ReplyMessageRequest,
};
use reqwest::{Client, Response};
use std::error::Error;
use std::fmt;

const LINE_API_BASE_URL: &str = "https://api.line.me/v2/bot";

#[derive(Debug)]
pub struct LineApiError {
    pub message: String,
    pub status_code: Option<u16>,
}

impl fmt::Display for LineApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LINE API Error: {}", self.message)
    }
}

impl Error for LineApiError {}

#[derive(Clone)]
pub struct LineApiClient {
    client: Client,
    channel_access_token: String,
}

impl LineApiClient {
    pub fn new(channel_access_token: String) -> Self {
        Self {
            client: Client::new(),
            channel_access_token,
        }
    }

    pub async fn reply_message(
        &self,
        reply_token: &str,
        messages: Vec<OutgoingMessage>,
    ) -> Result<(), LineApiError> {
        let request = ReplyMessageRequest {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled: None,
        };

        let url = format!("{}/message/reply", LINE_API_BASE_URL);
        let response = self.send_request(&url, &request).await?;
        self.handle_response(response).await
    }

    pub async fn push_message(
        &self,
        to: &str,
        messages: Vec<OutgoingMessage>,
    ) -> Result<(), LineApiError> {
        let request = PushMessageRequest {
            to: to.to_string(),
            messages,
            notification_disabled: None,
        };

        let url = format!("{}/message/push", LINE_API_BASE_URL);
        let response = self.send_request(&url, &request).await?;
        self.handle_response(response).await
    }

    pub async fn multicast_message(
        &self,
        to: Vec<String>,
        messages: Vec<OutgoingMessage>,
    ) -> Result<(), LineApiError> {
        let request = MulticastMessageRequest {
            to,
            messages,
            notification_disabled: None,
        };

        let url = format!("{}/message/multicast", LINE_API_BASE_URL);
        let response = self.send_request(&url, &request).await?;
        self.handle_response(response).await
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<serde_json::Value, LineApiError> {
        let url = format!("{}/profile/{}", LINE_API_BASE_URL, user_id);

        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.channel_access_token),
            )
            .send()
            .await
            .map_err(|e| LineApiError {
                message: format!("Failed to send request: {}", e),
                status_code: None,
            })?;

        if response.status().is_success() {
            let profile = response.json().await.map_err(|e| LineApiError {
                message: format!("Failed to parse profile response: {}", e),
                status_code: None,
            })?;
            Ok(profile)
        } else {
            let status_code = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(LineApiError {
                message: format!("Profile API error: {}", error_text),
                status_code: Some(status_code),
            })
        }
    }

    async fn send_request<T: serde::Serialize>(
        &self,
        url: &str,
        request: &T,
    ) -> Result<Response, LineApiError> {
        self.client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.channel_access_token),
            )
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await
            .map_err(|e| LineApiError {
                message: format!("Failed to send request: {}", e),
                status_code: None,
            })
    }

    async fn handle_response(&self, response: Response) -> Result<(), LineApiError> {
        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status().as_u16();
            let error_response: ApiResponse = response.json().await.map_err(|e| LineApiError {
                message: format!("Failed to parse error response: {}", e),
                status_code: Some(status_code),
            })?;

            let error_message = error_response.message.unwrap_or_else(|| {
                error_response
                    .details
                    .map(|details| {
                        details
                            .into_iter()
                            .map(|e| e.message)
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "Unknown error".to_string())
            });

            Err(LineApiError {
                message: error_message,
                status_code: Some(status_code),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_api_client_creation() {
        let client = LineApiClient::new("test_token".to_string());
        assert_eq!(client.channel_access_token, "test_token");
    }
}
