use config::{Config, ConfigError as ConfigBuildError, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct RaydarConfig {
    pub swap_amount: f64,
    pub swap_fee_amount: f64,
    pub swap_hold_time: u64,
}

impl RaydarConfig {
    fn new() -> Self {}
}
