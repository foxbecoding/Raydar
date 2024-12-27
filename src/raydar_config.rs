use config::{Config, File};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone, Debug)]
pub struct RaydarConfig {
    pub swap_amount: f64,
    pub swap_fee_amount: f64,
    pub swap_hold_time: u64,
}

impl RaydarConfig {
    pub fn new() -> Self {
        let config_filename =
            env::var("CONFIG_FILENAME").expect("CONFIG_FILENAME environment variable not set");

        let settings = Config::builder()
            .add_source(File::with_name(&config_filename))
            .build()
            .expect("Failed to access config fail");

        let config: Self = settings
            .try_deserialize()
            .expect("Failed to deserialize raydar config");
        config
    }
}
