use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub log_level: Option<String>,
}

fn default_log_level() -> Option<String> {
    Some("info".to_string())
}

impl Config {
    pub fn try_load(path: &'static str) -> anyhow::Result<Self> {
        let file = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("failed to read config file at {}: {:?}", path, e))?;
        toml::from_str::<Config>(&file)
            .map_err(|e| anyhow::anyhow!("failed to parse config file at {}: {:?}", path, e))
    }
}
