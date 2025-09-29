use rust_service::service::{Action, Config, ServiceError};
use chrono::Utc;
use log::info;

pub struct TimeAction;

impl Action<Config> for TimeAction {
    fn execute(&self, _config: &Config) -> Result<(), ServiceError> {
        let CURRENT_TIME = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        println!("Current time: {CURRENT_TIME}");
        info!("Current time: {CURRENT_TIME}");
        Ok(())
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
