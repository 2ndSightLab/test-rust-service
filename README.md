# test-rust-service

A sample Rust service that demonstrates how to use the rust-service library to build a secure, production-ready service. This service implements a TimeAction that prints the current time with comprehensive logging and configuration management.

**Updated for rust-service v0.1.0**: This project has been updated to work with the new rust-service library structure that separates the service framework from the executable implementation.

__Building and Testing__

```bash
# Build
./scripts/build.sh

# Run tests
./scripts/test.sh

# Check best practices
./scripts/best-practices.sh

# Install the program
./scripts/install.sh

# Run service
./scripts/run.sh
```

__Blog Posts__

Written in one day having never used rust before:\
https://medium.com/cloud-security/how-i-learned-rust-in-one-day-with-amazon-q-1398b75270c5

Fixed on the second day to make it more production-ready by fixing security vulnerabilities and following rust best practices:\
https://medium.com/cloud-security/getting-amazon-q-to-help-write-production-ready-rust-code-1b3146338bad

Checking security vulnerabilities with Amazon Q:\
https://medium.com/cloud-security/using-ai-to-check-for-security-vulnerabilities-across-your-code-base-fcd48e246d04

Turning AI security findings into repeatable, deterministic security checks:\
https://medium.com/cloud-security/turn-your-security-findings-into-automated-checks-0a08efe57358

Preventing Q from making the same mistakes over and over again:\
https://medium.com/cloud-security/preventing-amazon-q-from-making-the-same-mistakes-over-and-over-4220c4c1a356

Turning the service into an extensible service library anyone can use that runs their own actions:\
https://medium.com/cloud-security/an-extensible-library-anyone-can-use-to-build-a-rust-service-f88eddf9d14f

__Features__

- **Time Action**: Prints current UTC timestamp
- **Configuration Management**: TOML-based configuration with validation
- **Graceful Shutdown**: Handles Ctrl+C for clean service termination
- **Extensible Framework**: Built on rust-service library for easy extension

__Implementation__

This service implements a `TimeAction` in `src/action/exec.rs`:

```rust
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
```

__Configuration__

The service uses the rust-service library's configuration system. Action-specific configuration is defined in `config/action.toml`:

```toml
MESSAGE = "Current time: {}"
MAX_MESSAGE_LEN = 100
TIME_INTERVAL = 5
MAX_TIME_INTERVAL = 3600
DEFAULT_MESSAGE_LEN = 50
CURRENT_TIME_FORMAT = "%Y-%m-%d %H:%M:%S UTC"
MESSAGE_FORMAT = "Current time: {}"
```

Configuration files are expected in standard locations:
- Service config: `service.toml` 
- Action config: `action.toml`

__Dependencies__

- `rust-service`: Core service framework library
- `chrono`: Date/time handling for timestamp generation
- `log`: Logging framework
- `serde`: Configuration serialization
- `toml`: Configuration file parsing
- `ctrlc`: Signal handling for graceful shutdown
- `libc`: System interface

__Security Features__

- Dedicated service user (test-rust-service)
- Protected configuration files (root-owned, 644 permissions)
- Input validation and sanitization
- System resource monitoring
- Secure file operations with proper locking
- Prevention of root execution

__Installation Layout__

- Binary: `/opt/test-rust-service/test-rust-service`
- Config: `/etc/test-rust-service/config-service.toml` and `/etc/test-rust-service/config-action.toml`
- Logs: `/var/log/test-rust-service/`
- Service User: `test-rust-service` (system account)

__Usage Example__

Run all the scripts under Building and Testing in order.

After installation, the service runs continuously and outputs:
```
Current time: 2025-10-06 20:19:29 UTC
Current time: 2025-10-06 20:19:34 UTC
Current time: 2025-10-06 20:19:39 UTC
```

Stop with Ctrl+C for graceful shutdown.
