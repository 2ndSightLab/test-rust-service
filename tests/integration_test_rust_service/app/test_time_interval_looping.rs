use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn test_time_interval_looping() {
    // Prevent running as root
    assert!(
        (unsafe { libc::getuid() } != 0),
        "This test must not be run as root"
    );

    // Start the service in a separate process
    let mut CHILD =
        Command::new("/opt/test-rust-service-debug/test-rust-service-debug/test-rust-service")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start service");

    let STDOUT = CHILD.stdout.take().expect("Failed to capture stdout");
    let mut READER = BufReader::new(STDOUT);

    let mut TIMESTAMPS = Vec::new();
    let START_TIME = Instant::now();
    let TEST_DURATION = Duration::from_secs(12); // Test for 12 seconds to capture at least 2 intervals

    // Read stdout lines and capture timestamps when we see time output
    let mut LINE = String::new();
    while START_TIME.elapsed() < TEST_DURATION {
        LINE.clear();
        match READER.read_line(&mut LINE) {
            Ok(0) | Err(_) => break, // EOF or error
            Ok(_) => {
                if LINE.contains("Current time:") {
                    TIMESTAMPS.push(Instant::now());
                    if TIMESTAMPS.len() >= 3 {
                        break; // We have enough samples
                    }
                }
            }
        }
    }

    // Kill the child process
    let _ = CHILD.kill();
    let _ = CHILD.wait();

    // Verify we got at least 2 timestamps
    assert!(
        TIMESTAMPS.len() >= 2,
        "Should have captured at least 2 time outputs on stdout, got {}",
        TIMESTAMPS.len()
    );

    // Verify the interval is approximately 5 seconds (allow 1 second tolerance)
    for i in 1..TIMESTAMPS.len() {
        let INTERVAL = TIMESTAMPS[i].duration_since(TIMESTAMPS[i - 1]);
        let INTERVAL_SECS = INTERVAL.as_secs();
        assert!(
            (4..=6).contains(&INTERVAL_SECS),
            "Time interval should be approximately 5 seconds, got {INTERVAL_SECS} seconds"
        );
    }
}
