use chrono::Utc;
use log::info;
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
        loop {
            let CURRENT_TIME = Utc::now().format(DEFAULT_TIME_FORMAT);
            let MESSAGE = DEFAULT_MESSAGE_FORMAT.replace("{}", &CURRENT_TIME.to_string());

            println!("{MESSAGE}");
            info!("{MESSAGE}");

            thread::sleep(Duration::from_secs(DEFAULT_TIME_INTERVAL));
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
