use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    pub environment: String,
    pub workers: usize,
    pub database: Database,
    twitter: Twitter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let app_environment =
            std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Layered config, each chained source can override the prior one.
        Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{app_environment}")).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize()
    }
}

lazy_static::lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new()
    .unwrap_or_else(|error| panic!("Fatal configuration error: {}", error));
}

#[cfg(test)]
mod test {
    use super::Settings;

    #[test]
    fn test_loads_test_config() {
        std::env::set_var("APP_ENVIRONMENT", "test");

        let settings =
            Settings::new().unwrap_or_else(|error| panic!("Fatal configuration error: {}", error));

        assert_eq!(settings.database.url, "postgres://test.url");
    }
}
