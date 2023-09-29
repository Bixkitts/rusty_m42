use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub auth_token: String,
    pub api_endpoint: String,
    pub raw_token: Option<String>,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open("config.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        #[derive(Deserialize)]
        struct TempConfig {
            auth_token: String,
            api_endpoint: String,
        }
        let temp_config: TempConfig = serde_json::from_str(&contents)?;
        let config = Config {
            auth_token: temp_config.auth_token,
            api_endpoint: temp_config.api_endpoint,
            raw_token: None,
        };

        Ok(config)
    }
    pub fn set_raw_token(&mut self, string: &str) {
        self.raw_token = Some(string.to_string());
    }
}