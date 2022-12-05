use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

pub trait ConfigFromEnv {
    fn from_env() -> Result<Config>;
}

impl ConfigFromEnv for Config {
    fn from_env() -> Result<Config> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        return cfg.try_into()
            .context("Loading configuration from environment");
    }
}
