use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;
use tracing::{debug, warn};

/// 速率限制條目
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    last_reset: Instant,
    last_request: Instant,
}

impl RateLimitEntry {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            count: 1,
            last_reset: now,
            last_request: now,
        }
    }

    fn increment(&mut self, window_duration: Duration) -> bool {
        let now = Instant::now();

        // 重置窗口
        if now.duration_since(self.last_reset) >= window_duration {
            self.count = 1;
            self.last_reset = now;
            self.last_request = now;
            return true;
        }

        self.count += 1;
        self.last_request = now;
        true
    }

    fn time_until_reset(&self, window_duration: Duration) -> Duration {
        let elapsed = Instant::now().duration_since(self.last_reset);
        window_duration.saturating_sub(elapsed)
    }
}

/// 速率限制器配置
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_duration: Duration,
    pub cleanup_interval: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 10,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300), // 5 分鐘
        }
    }
}

/// 速率限制器
#[derive(Debug, Clone)]
pub struct RateLimiter {
    entries: Arc<DashMap<String, RateLimitEntry>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        let rate_limiter = Self {
            entries: Arc::new(DashMap::new()),
            config,
        };

        // 啟動清理任務
        let cleanup_entries = rate_limiter.entries.clone();
        let cleanup_config = rate_limiter.config.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_config.cleanup_interval);
            loop {
                interval.tick().await;
                Self::cleanup_expired_entries(&cleanup_entries, &cleanup_config);
            }
        });

        rate_limiter
    }

    pub fn check_rate_limit(&self, key: &str) -> RateLimitResult {
        let now = Instant::now();

        if let Some(mut entry) = self.entries.get_mut(key) {
            // 檢查是否需要重置窗口
            if now.duration_since(entry.last_reset) >= self.config.window_duration {
                *entry = RateLimitEntry::new();
                return RateLimitResult::Allowed {
                    remaining: self.config.max_requests - 1,
                    reset_after: self.config.window_duration,
                };
            }

            // 檢查是否超過限制
            if entry.count >= self.config.max_requests {
                warn!("Rate limit exceeded for key: {}", key);
                return RateLimitResult::Exceeded {
                    retry_after: entry.time_until_reset(self.config.window_duration),
                };
            }

            entry.increment(self.config.window_duration);
            RateLimitResult::Allowed {
                remaining: self.config.max_requests.saturating_sub(entry.count),
                reset_after: entry.time_until_reset(self.config.window_duration),
            }
        } else {
            // 新條目
            self.entries.insert(key.to_string(), RateLimitEntry::new());
            RateLimitResult::Allowed {
                remaining: self.config.max_requests - 1,
                reset_after: self.config.window_duration,
            }
        }
    }

    fn cleanup_expired_entries(
        entries: &DashMap<String, RateLimitEntry>,
        config: &RateLimitConfig,
    ) {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        for entry in entries.iter() {
            let key = entry.key();
            let value = entry.value();

            // 移除超過窗口時間且沒有最近請求的條目
            if now.duration_since(value.last_reset) > config.window_duration
                && now.duration_since(value.last_request) > config.window_duration
            {
                to_remove.push(key.clone());
            }
        }

        for key in to_remove {
            entries.remove(&key);
            debug!("Cleaned up expired rate limit entry: {}", key);
        }
    }
}

/// 速率限制結果
#[derive(Debug)]
pub enum RateLimitResult {
    Allowed {
        remaining: u32,
        reset_after: Duration,
    },
    Exceeded {
        retry_after: Duration,
    },
}

/// 從請求中提取速率限制鍵
fn extract_rate_limit_key(request: &Request) -> String {
    // 優先使用 X-Forwarded-For 頭
    if let Some(forwarded_for) = request.headers().get("x-forwarded-for")
        && let Ok(forwarded_str) = forwarded_for.to_str()
        && let Some(first_ip) = forwarded_str.split(',').next()
    {
        return first_ip.trim().to_string();
    }

    // 使用 X-Real-IP 頭
    if let Some(real_ip) = request.headers().get("x-real-ip")
        && let Ok(ip_str) = real_ip.to_str()
    {
        return ip_str.to_string();
    }

    // 使用連接信息中的 IP
    if let Some(ConnectInfo(addr)) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    // 預設值
    "unknown".to_string()
}

/// 速率限制中介軟體
pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {
    // 從應用狀態獲取速率限制器
    // 這裡需要從應用狀態中提取 RateLimiter
    let rate_limiter = RateLimiter::new(RateLimitConfig::default());

    let key = extract_rate_limit_key(&request);

    match rate_limiter.check_rate_limit(&key) {
        RateLimitResult::Allowed {
            remaining,
            reset_after,
        } => {
            let mut response = next.run(request).await;
            let headers = response.headers_mut();
            headers.insert(
                "X-RateLimit-Remaining",
                remaining.to_string().parse().unwrap(),
            );
            headers.insert(
                "X-RateLimit-Reset",
                reset_after.as_secs().to_string().parse().unwrap(),
            );
            response
        }
        RateLimitResult::Exceeded { retry_after } => {
            let retry_after_secs = retry_after.as_secs();
            warn!(
                "Rate limit exceeded for {}, retry after {} seconds",
                key, retry_after_secs
            );

            let mut response = StatusCode::TOO_MANY_REQUESTS.into_response();
            let headers = response.headers_mut();
            headers.insert("Retry-After", retry_after_secs.to_string().parse().unwrap());
            headers.insert("X-RateLimit-Remaining", "0".parse().unwrap());
            response
        }
    }
}

/// 創建帶有自定義配置的速率限制中介軟體
pub fn create_rate_limit_middleware(
    config: RateLimitConfig,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
+ Clone {
    let rate_limiter = Arc::new(RateLimiter::new(config));

    move |request: Request, next: Next| {
        let rate_limiter = rate_limiter.clone();
        Box::pin(async move {
            let key = extract_rate_limit_key(&request);

            match rate_limiter.check_rate_limit(&key) {
                RateLimitResult::Allowed {
                    remaining,
                    reset_after,
                } => {
                    let mut response = next.run(request).await;
                    let headers = response.headers_mut();
                    headers.insert(
                        "X-RateLimit-Remaining",
                        remaining.to_string().parse().unwrap(),
                    );
                    headers.insert(
                        "X-RateLimit-Reset",
                        reset_after.as_secs().to_string().parse().unwrap(),
                    );
                    response
                }
                RateLimitResult::Exceeded { retry_after } => {
                    let retry_after_secs = retry_after.as_secs();
                    warn!(
                        "Rate limit exceeded for {}, retry after {} seconds",
                        key, retry_after_secs
                    );

                    // 添加延遲以減緩攻擊
                    sleep(Duration::from_millis(100)).await;

                    let mut response = StatusCode::TOO_MANY_REQUESTS.into_response();
                    let headers = response.headers_mut();
                    headers.insert("Retry-After", retry_after_secs.to_string().parse().unwrap());
                    headers.insert("X-RateLimit-Remaining", "0".parse().unwrap());
                    response
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_rate_limit_entry_creation() {
        let entry = RateLimitEntry::new();
        assert_eq!(entry.count, 1);
    }

    #[test]
    fn test_rate_limit_entry_increment() {
        let mut entry = RateLimitEntry::new();
        let window = Duration::from_secs(60);

        entry.increment(window);
        assert_eq!(entry.count, 2);

        // 模擬多次請求
        for _ in 0..8 {
            entry.increment(window);
        }
        assert_eq!(entry.count, 10);

        entry.increment(window);
        assert_eq!(entry.count, 11);
    }

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let config = RateLimitConfig {
            max_requests: 5,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
        };

        let limiter = RateLimiter::new(config);
        let key = "test_key";

        // 前5次請求應該被允許
        for i in 1..=5 {
            match limiter.check_rate_limit(key) {
                RateLimitResult::Allowed { remaining, .. } => {
                    assert_eq!(remaining, 5 - i);
                }
                RateLimitResult::Exceeded { .. } => panic!("Should be allowed"),
            }
        }

        // 第6次請求應該被拒絕
        match limiter.check_rate_limit(key) {
            RateLimitResult::Exceeded { .. } => {} // 預期結果
            RateLimitResult::Allowed { .. } => panic!("Should be exceeded"),
        }
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.max_requests, 10);
        assert_eq!(config.window_duration, Duration::from_secs(60));
        assert_eq!(config.cleanup_interval, Duration::from_secs(300));
    }
}
