use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub channel_access_token: String,
    pub channel_secret: String,
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let channel_access_token = env::var("CHANNEL_ACCESS_TOKEN")
            .map_err(|_| "CHANNEL_ACCESS_TOKEN environment variable is required")?;

        let channel_secret = env::var("CHANNEL_SECRET")
            .map_err(|_| "CHANNEL_SECRET environment variable is required")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number")?;

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        Ok(Config {
            channel_access_token,
            channel_secret,
            port,
            host,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        unsafe {
            env::set_var("CHANNEL_ACCESS_TOKEN", "test_token");
            env::set_var("CHANNEL_SECRET", "test_secret");
            env::set_var("PORT", "8080");
            env::set_var("HOST", "127.0.0.1");
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.channel_access_token, "test_token");
        assert_eq!(config.channel_secret, "test_secret");
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "127.0.0.1");
    }
}
