use axum::{extract::Request, middleware::Next, response::Response};
use metrics::{counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram};
use std::time::Instant;

/// 初始化指標收集系統
pub fn init_metrics() {
    // 描述指標
    describe_counter!("http_requests_total", "Total number of HTTP requests");
    describe_counter!(
        "line_api_requests_total",
        "Total number of LINE API requests"
    );
    describe_counter!(
        "webhook_events_total",
        "Total number of webhook events processed"
    );
    describe_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds"
    );
    describe_histogram!(
        "line_api_duration_seconds",
        "LINE API request duration in seconds"
    );
    describe_gauge!("active_connections", "Number of active connections");
}

/// 設定 Prometheus 匯出器
#[cfg(feature = "metrics")]
pub fn setup_prometheus_exporter(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    use metrics_exporter_prometheus::PrometheusBuilder;

    let builder = PrometheusBuilder::new().with_http_listener(([0, 0, 0, 0], port));

    builder.install()?;
    Ok(())
}

#[cfg(not(feature = "metrics"))]
pub fn setup_prometheus_exporter(_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // 當沒有啟用 metrics 特性時，不做任何事
    Ok(())
}

/// HTTP 請求指標收集中介軟體
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().path().to_string();

    // 增加請求計數
    counter!("http_requests_total", "method" => method.to_string(), "endpoint" => uri.clone())
        .increment(1);

    let response = next.run(request).await;

    // 記錄回應時間
    let duration = start.elapsed();
    let status = response.status().as_u16().to_string();

    histogram!("http_request_duration_seconds", "method" => method.to_string(), "endpoint" => uri, "status" => status).record(duration.as_secs_f64());

    response
}

/// 記錄 Webhook 事件指標
pub fn record_webhook_event(event_type: &str) {
    counter!("webhook_events_total", "type" => event_type.to_string()).increment(1);
}

/// 記錄 LINE API 請求指標
pub fn record_line_api_request(api_type: &str, duration: std::time::Duration, success: bool) {
    let status = if success { "success" } else { "error" };

    counter!("line_api_requests_total", "api" => api_type.to_string(), "status" => status.to_string()).increment(1);
    histogram!("line_api_duration_seconds", "api" => api_type.to_string(), "status" => status.to_string()).record(duration.as_secs_f64());
}

/// 系統指標收集器
pub struct SystemMetrics {
    start_time: Instant,
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// 更新系統指標
    pub fn update_system_metrics(&self) {
        // 運行時間
        let uptime = self.start_time.elapsed().as_secs() as f64;
        gauge!("uptime_seconds").set(uptime);

        // 記憶體使用量 (簡化版本)
        if let Ok(memory_usage) = get_memory_usage() {
            gauge!("memory_usage_bytes").set(memory_usage as f64);
        }

        // 活躍連接數 (這裡是示例，實際需要從應用狀態獲取)
        // gauge!("active_connections").set(active_connections as f64);
    }

    /// 啟動定期指標更新任務
    pub fn start_periodic_update(&self) {
        let metrics = SystemMetrics::new();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                metrics.update_system_metrics();
            }
        });
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// 取得記憶體使用量 (簡化實作)
fn get_memory_usage() -> Result<u64, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status")?;
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let kb: u64 = parts[1].parse()?;
                    return Ok(kb * 1024); // 轉換為位元組
                }
            }
        }
    }

    #[cfg(windows)]
    {
        // Windows 的記憶體使用量取得需要額外的依賴
        // 這裡返回一個模擬值
        Ok(50 * 1024 * 1024) // 50MB
    }

    #[cfg(not(any(unix, windows)))]
    Err("Unable to get memory usage".into())
}

/// LINE Bot 特定指標
pub struct LineBotMetrics {
    pub total_users: u64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub api_errors: u64,
}

impl LineBotMetrics {
    pub fn new() -> Self {
        Self {
            total_users: 0,
            total_messages_sent: 0,
            total_messages_received: 0,
            api_errors: 0,
        }
    }

    pub fn increment_users(&mut self) {
        self.total_users += 1;
        gauge!("line_bot_users_total").set(self.total_users as f64);
    }

    pub fn increment_messages_sent(&mut self) {
        self.total_messages_sent += 1;
        counter!("line_bot_messages_sent_total").increment(1);
    }

    pub fn increment_messages_received(&mut self) {
        self.total_messages_received += 1;
        counter!("line_bot_messages_received_total").increment(1);
    }

    pub fn increment_api_errors(&mut self) {
        self.api_errors += 1;
        counter!("line_bot_api_errors_total").increment(1);
    }

    pub fn update_gauges(&self) {
        gauge!("line_bot_users_total").set(self.total_users as f64);
        gauge!("line_bot_api_errors_total").set(self.api_errors as f64);
    }
}

impl Default for LineBotMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// 健康檢查指標
pub fn record_health_check(healthy: bool) {
    let status = if healthy { "healthy" } else { "unhealthy" };
    counter!("health_checks_total", "status" => status.to_string()).increment(1);
    gauge!("service_healthy").set(if healthy { 1.0 } else { 0.0 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_metrics_creation() {
        let metrics = SystemMetrics::new();
        let uptime = metrics.start_time.elapsed();
        assert!(uptime.as_millis() < 100); // 應該是很新的
    }

    #[test]
    fn test_line_bot_metrics() {
        let mut metrics = LineBotMetrics::new();

        assert_eq!(metrics.total_users, 0);
        metrics.increment_users();
        assert_eq!(metrics.total_users, 1);

        assert_eq!(metrics.total_messages_sent, 0);
        metrics.increment_messages_sent();
        assert_eq!(metrics.total_messages_sent, 1);

        assert_eq!(metrics.total_messages_received, 0);
        metrics.increment_messages_received();
        assert_eq!(metrics.total_messages_received, 1);
    }

    #[test]
    fn test_init_metrics() {
        // 這個測試只是確保函數可以呼叫而不會 panic
        init_metrics();
    }
}
