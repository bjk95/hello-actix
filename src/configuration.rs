use config;
use config::ConfigError;
use serde::Deserialize;

pub fn get_config() -> Config {
    let mut cfg = config::Config::new();
    cfg.merge(config::File::with_name("./config/config.toml"))
        .unwrap();

    let cfg_as_struct: Result<Config, ConfigError> = cfg.try_into();

    cfg_as_struct.unwrap()
}

#[derive(Deserialize)]
pub struct Config {
    pub elastic_url: String,
}
