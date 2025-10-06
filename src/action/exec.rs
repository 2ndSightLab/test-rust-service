use chrono::Utc;
use log::info;
use rust_service::load_action_config;
use rust_service::service::{Action, Config, ServiceError};
use std::thread;
use std::time::Duration;

// Configuration variables
const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S UTC";
const DEFAULT_MESSAGE_FORMAT: &str = "Current time: {}";
const DEFAULT_TIME_INTERVAL: u64 = 5;

pub struct TimeAction;

impl Action<Config> for TimeAction {
    fn execute(&self, _config: &Config) -> Result<(), ServiceError> {
        let ACTION_CONFIG = load_action_config()?;
        let CURRENT_TIME_FORMAT: String = ACTION_CONFIG
            .get("CURRENT_TIME_FORMAT")
            .unwrap_or_else(|| DEFAULT_TIME_FORMAT.to_string());
        let MESSAGE_FORMAT: String = ACTION_CONFIG
            .get("MESSAGE_FORMAT")
            .unwrap_or_else(|| DEFAULT_MESSAGE_FORMAT.to_string());
        let TIME_INTERVAL: u64 = ACTION_CONFIG
            .get("TIME_INTERVAL")
            .unwrap_or(DEFAULT_TIME_INTERVAL);

        loop {
            let CURRENT_TIME = Utc::now().format(&CURRENT_TIME_FORMAT);
            let MESSAGE = MESSAGE_FORMAT.replace("{}", &CURRENT_TIME.to_string());

            println!("{MESSAGE}");
            info!("{MESSAGE}");

            thread::sleep(Duration::from_secs(TIME_INTERVAL));
        }
    }

    fn name(&self) -> &'static str {
        "time"
    }
}

/// # Errors
/// Never returns an error
pub fn new() -> Result<Box<dyn Action<Config>>, ServiceError> {
    Ok(Box::new(TimeAction))
}
