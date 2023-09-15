use config_rs::{Config, File};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub db_url: String,
    pub http_port: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: false,
            db_url: Default::default(),
            http_port: "8800".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        let run_mode = dotenv::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Add in `./Settings.toml`
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode))) // merge two file as config
            .build()?;

        Ok(config.try_deserialize::<Self>()?)
    }
}
