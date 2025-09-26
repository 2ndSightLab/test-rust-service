# test-rust-service

A sample Rust service that demonstrates how to use the [rust-service](https://github.com/2ndSightLab/rust-service) library to build a secure, production-ready service. This service prints the current time periodically with comprehensive logging, configuration management, and security validation.

## Blog Posts

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

## Features

- **Time Action**: Prints current UTC timestamp every configured interval
- **Security Validation**: Prevents running as root, validates user identity
- **System Monitoring**: Monitors memory and disk usage thresholds
- **Secure Logging**: File logging with proper permissions and locking
- **Configuration Management**: TOML-based configuration with validation
- **Graceful Shutdown**: Handles Ctrl+C for clean service termination

## Architecture

This service implements a single `TimeAction` that uses the rust-service library framework:

```rust
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
```

## Configuration

The service uses `/etc/test-rust-service/config.toml` with these settings:

- `SERVICE_NAME`: Service identifier (test-rust-service)
- `TIME_INTERVAL`: Seconds between time outputs (default: 5)
- `MESSAGE`: Service status message
- `LOG_FILE_PATH`: Log directory path (/var/log/test-rust-service)
- `MEMORY_THRESHOLD`: Memory usage alert threshold (default: 80%)
- `DISK_THRESHOLD`: Disk usage alert threshold (default: 80%)
- Additional security and validation limits

## Building and Running

### Build
```bash
./build.sh
```
Choose between debug (1) or release (2) build.

### Install
```bash
./install.sh
```
Creates service user, directories, and installs binary with proper permissions.

### Run
```bash
./run.sh
```
Validates installation and runs service as dedicated user.

### Test
```bash
./test.sh
```
Runs comprehensive test suite.

### Security Check
```bash
./best-practices.sh
```
Validates code follows Rust security best practices.

## Dependencies

- `rust-service`: Core service framework library
- `chrono`: Date/time handling for timestamp generation
- `log`: Logging framework
- `serde`: Configuration serialization
- `toml`: Configuration file parsing
- Security and system libraries (users, nix, libc)

## Security Features

- Dedicated service user (test-rust-service)
- Protected configuration files (root-owned, 644 permissions)
- Input validation and sanitization
- System resource monitoring
- Secure file operations with proper locking
- Prevention of root execution

## Installation Layout

- Binary: `/opt/test-rust-service/test-rust-service`
- Config: `/etc/test-rust-service/config.toml`
- Logs: `/var/log/test-rust-service/`
- Service User: `test-rust-service` (system account)

## Usage Example

After installation, the service runs continuously and outputs:
```
Current time: 2025-09-26 20:09:31 UTC
Current time: 2025-09-26 20:09:36 UTC
Current time: 2025-09-26 20:09:41 UTC
```

Stop with Ctrl+C for graceful shutdown.
