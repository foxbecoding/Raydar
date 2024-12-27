use config::{Config, ConfigError as ConfigBuildError, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct RaydarConfig {
    pub swap_amount: f64,
    pub swap_fee_amount: f64,
    pub swap_hold_time: u64,
}

impl RaydarConfig {
    fn new() -> Self {
        let settings = Config::builder()
            .add_source(File::with_name("raydar_config.toml"))
            .build()
            .expect("Failed to access config fail");

        let config: Self = settings
            .try_deserialize()
            .expect("Failed to deserialize raydar config");
        config
    }
}
