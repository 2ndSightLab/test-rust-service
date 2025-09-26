#!/bin/bash
set -e

# First run the install.sh script
./install.sh

# Validate target user exists and has correct setup
if ! id "test-rust-service" &>/dev/null; then
    echo "Error: test-rust-service user does not exist"
    exit 1
fi

# Verify binary exists and is executable
if [[ ! -x "/opt/test-rust-service/test-rust-service" ]]; then
    echo "Error: Service binary not found or not executable"
    exit 1
fi

# Verify config file exists and has correct permissions
if [[ ! -f "/etc/test-rust-service/config.toml" ]]; then
    echo "Error: Config file not found"
    exit 1
fi

# Check config file permissions (should not be world-writable)
if [[ $(stat -c "%a" "/etc/test-rust-service/config.toml") -gt 644 ]]; then
    echo "Error: Config file has insecure permissions"
    exit 1
fi

# Run the program with validated user
sudo -u test-rust-service /opt/test-rust-service/test-rust-service
