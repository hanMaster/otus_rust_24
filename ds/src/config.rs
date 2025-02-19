use std::env;
use std::sync::OnceLock;
use crate::error::Result;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub recv_window: String,
    pub api_url: String
}

impl Config {
    fn load_from_env() -> Result<Config> {
        dotenvy::dotenv()?;
        let api_key = env::var("API_KEY")?;
        let secret_key = env::var("SECRET_KEY")?;
        let recv_window = env::var("RECV_WINDOW")?;
        let api_url = env::var("API_URL")?;
        Ok(Config {
            api_key,
            secret_key,
            recv_window,
            api_url
        })
    }
}
