use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // 默认配置
            .add_source(File::with_name("config/default"))
            // 特定环境配置
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // 环境变量
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_settings() {
        env::set_var("APP_DATABASE_URL", "postgres://test:test@localhost/test_db");
        env::set_var("APP_SERVER_HOST", "127.0.0.1");
        env::set_var("APP_SERVER_PORT", "8080");

        let settings = Settings::new().unwrap();
        assert_eq!(settings.database_url, "postgres://test:test@localhost/test_db");
        assert_eq!(settings.server_host, "127.0.0.1");
        assert_eq!(settings.server_port, 8080);
    }
}