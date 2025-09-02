use linebot_rs::{Config, start_server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "linebot_rs=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    println!("Starting LINE Bot server...");
    println!(
        "Channel Access Token configured: {}",
        !config.channel_access_token.is_empty()
    );
    println!(
        "Channel Secret configured: {}",
        !config.channel_secret.is_empty()
    );
    println!("Server will listen on: {}:{}", config.host, config.port);

    start_server(config).await?;

    Ok(())
}
