use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::load();
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app_info: AppInfoConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        let e = std::env::var("rust_env").unwrap_or("dev".to_string());
        let file_name = format!("config/config.{}.toml",e);
        let s = Config::builder()
            .add_source(config::File::with_name(file_name.as_str()))
            .build()
            .unwrap();
        s.try_deserialize().unwrap()
    }
    
}

#[derive(Debug, Deserialize)]
pub struct AppInfoConfig {
    pub app_name: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub dsn: String,
}

#[derive(Debug, Deserialize,Default)]
pub struct LogConfig {
    #[serde(default="default_level")]
    pub level: String,
}

fn default_level()->String{
    "info".to_string()
}

#[test]
fn test_config() {
    let c = AppConfig::load();
    assert_eq!(c.app_info.app_name, "axum_blog");
    assert_eq!(c.log.level, "info");
}
