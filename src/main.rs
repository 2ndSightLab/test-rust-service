use rust_service::{Action, ServiceRunner, ServiceError, config::Config};
use log::info;
use chrono::Utc;

struct TimeAction;

impl Action for TimeAction {
    fn execute(&self, _config: &Config) -> Result<(), ServiceError> {
        let CURRENT_TIME = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        info!("Current time: {}", CURRENT_TIME);
        Ok(())
    }

    fn name(&self) -> &str {
        "time"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ServiceRunner::new()
        .add_action(Box::new(TimeAction))
        .run()
}
