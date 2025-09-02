use std::collections::HashSet;

/// 輸入驗證錯誤
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooLong { max_length: usize, actual: usize },
    TooShort { min_length: usize, actual: usize },
    InvalidCharacters,
    Forbidden,
    Empty,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TooLong { max_length, actual } => {
                write!(f, "Input too long: {} > {}", actual, max_length)
            }
            ValidationError::TooShort { min_length, actual } => {
                write!(f, "Input too short: {} < {}", actual, min_length)
            }
            ValidationError::InvalidCharacters => write!(f, "Contains invalid characters"),
            ValidationError::Forbidden => write!(f, "Contains forbidden content"),
            ValidationError::Empty => write!(f, "Input is empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// 文字訊息驗證器
pub struct TextValidator {
    max_length: usize,
    min_length: usize,
    forbidden_words: HashSet<String>,
    allow_empty: bool,
}

impl Default for TextValidator {
    fn default() -> Self {
        let mut forbidden_words = HashSet::new();
        // 添加一些常見的禁用詞彙
        forbidden_words.insert("spam".to_string());
        forbidden_words.insert("垃圾".to_string());
        forbidden_words.insert("廣告".to_string());

        Self {
            max_length: 2000,
            min_length: 0,
            forbidden_words,
            allow_empty: true,
        }
    }
}

impl TextValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn add_forbidden_word(mut self, word: &str) -> Self {
        self.forbidden_words.insert(word.to_lowercase());
        self
    }

    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    pub fn validate(&self, text: &str) -> Result<(), ValidationError> {
        // 檢查是否為空
        if text.is_empty() {
            if self.allow_empty {
                return Ok(());
            } else {
                return Err(ValidationError::Empty);
            }
        }

        // 檢查長度
        let text_len = text.chars().count();
        if text_len > self.max_length {
            return Err(ValidationError::TooLong {
                max_length: self.max_length,
                actual: text_len,
            });
        }

        if text_len < self.min_length {
            return Err(ValidationError::TooShort {
                min_length: self.min_length,
                actual: text_len,
            });
        }

        // 檢查禁用詞彙
        let text_lower = text.to_lowercase();
        for forbidden_word in &self.forbidden_words {
            if text_lower.contains(forbidden_word) {
                return Err(ValidationError::Forbidden);
            }
        }

        // 檢查控制字符
        if text
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
        {
            return Err(ValidationError::InvalidCharacters);
        }

        Ok(())
    }
}

/// 用戶 ID 驗證器
pub struct UserIdValidator;

impl UserIdValidator {
    pub fn validate(user_id: &str) -> Result<(), ValidationError> {
        if user_id.is_empty() {
            return Err(ValidationError::Empty);
        }

        // LINE 用戶 ID 應該是 33 個字符
        if user_id.len() != 33 {
            return Err(ValidationError::InvalidCharacters);
        }

        // 應該以 U 開頭
        if !user_id.starts_with('U') {
            return Err(ValidationError::InvalidCharacters);
        }

        // 其餘字符應該是十六進制（包括大寫和小寫）
        for c in user_id.chars().skip(1) {
            if !c.is_ascii_hexdigit() {
                return Err(ValidationError::InvalidCharacters);
            }
        }

        Ok(())
    }
}

/// Reply Token 驗證器
pub struct ReplyTokenValidator;

impl ReplyTokenValidator {
    pub fn validate(reply_token: &str) -> Result<(), ValidationError> {
        if reply_token.is_empty() {
            return Err(ValidationError::Empty);
        }

        // Reply token 長度檢查
        let len = reply_token.len();
        if !(10..=100).contains(&len) {
            return Err(ValidationError::TooLong {
                max_length: 100,
                actual: len,
            });
        }

        // 應該只包含安全字符
        if !reply_token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(ValidationError::InvalidCharacters);
        }

        Ok(())
    }
}

/// 敏感資料遮罩工具
pub struct SensitiveDataMasker;

impl SensitiveDataMasker {
    /// 遮罩 Channel Access Token
    pub fn mask_channel_token(token: &str) -> String {
        if token.len() <= 8 {
            "*".repeat(token.len())
        } else {
            format!("{}...{}", &token[..4], "*".repeat(4))
        }
    }

    /// 遮罩用戶 ID
    pub fn mask_user_id(user_id: &str) -> String {
        if user_id.len() <= 6 {
            "*".repeat(user_id.len())
        } else {
            format!("{}...{}", &user_id[..3], &user_id[user_id.len() - 3..])
        }
    }

    /// 遮罩電子郵件
    pub fn mask_email(email: &str) -> String {
        if let Some(at_pos) = email.find('@') {
            let (local, domain) = email.split_at(at_pos);
            if local.len() <= 2 {
                format!("*@{}", domain)
            } else {
                format!("{}***@{}", &local[..1], &domain[1..])
            }
        } else {
            "*".repeat(email.len())
        }
    }

    /// 遮罩電話號碼
    pub fn mask_phone(phone: &str) -> String {
        if phone.len() <= 4 {
            "*".repeat(phone.len())
        } else {
            format!("{}***{}", &phone[..2], &phone[phone.len() - 2..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_validator_valid() {
        let validator = TextValidator::new();
        assert!(validator.validate("Hello World").is_ok());
        assert!(validator.validate("你好世界").is_ok());
        assert!(validator.validate("").is_ok()); // 允許空字串
    }

    #[test]
    fn test_text_validator_too_long() {
        let validator = TextValidator::new().max_length(5);
        let result = validator.validate("Hello World");
        assert!(matches!(result, Err(ValidationError::TooLong { .. })));
    }

    #[test]
    fn test_text_validator_forbidden_word() {
        let validator = TextValidator::new();
        let result = validator.validate("This is spam content");
        assert_eq!(result, Err(ValidationError::Forbidden));
    }

    #[test]
    fn test_text_validator_control_characters() {
        let validator = TextValidator::new();
        let result = validator.validate("Hello\x00World");
        assert_eq!(result, Err(ValidationError::InvalidCharacters));
    }

    #[test]
    fn test_user_id_validator_valid() {
        let valid_user_id = "U1234567890abcdef1234567890abcdef";
        assert!(UserIdValidator::validate(valid_user_id).is_ok());
    }

    #[test]
    fn test_user_id_validator_invalid_length() {
        assert!(UserIdValidator::validate("U123").is_err());
        assert!(UserIdValidator::validate("").is_err());
    }

    #[test]
    fn test_user_id_validator_invalid_format() {
        assert!(UserIdValidator::validate("X1234567890abcdef1234567890abcdef").is_err());
        assert!(UserIdValidator::validate("U123456789gabcdef1234567890abcdef").is_err());
    }

    #[test]
    fn test_reply_token_validator() {
        assert!(ReplyTokenValidator::validate("valid_reply_token_123").is_ok());
        assert!(ReplyTokenValidator::validate("").is_err());
        assert!(ReplyTokenValidator::validate("abc").is_err());
        assert!(ReplyTokenValidator::validate("invalid token with spaces").is_err());
    }

    #[test]
    fn test_sensitive_data_masker() {
        assert_eq!(
            SensitiveDataMasker::mask_channel_token("1234567890abcdef"),
            "1234...****"
        );
        assert_eq!(
            SensitiveDataMasker::mask_user_id("U1234567890abcdef1234567890abcdef"),
            "U12...def"
        );
        assert_eq!(
            SensitiveDataMasker::mask_email("test@example.com"),
            "t***@example.com"
        );
        assert_eq!(SensitiveDataMasker::mask_phone("0912345678"), "09***78");
    }
}
