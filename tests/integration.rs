use std::process::Command;

mod integration {
    mod time_output_test;
}

#[test]
fn run_all_common_integration_tests() {
    let output = Command::new("cargo")
        .args(&["test", "--manifest-path", "../rust-common-tests/Cargo.toml", "--", "--nocapture", "tests::integration::common::"])
        .current_dir(".")
        .output()
        .expect("Failed to execute integration tests");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print the output so we can see individual test results
    println!("{}", stdout);
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }
    
    // Check if any tests failed
    let failed_count = stdout.lines()
        .filter(|line| line.contains("test result:"))
        .filter_map(|line| {
            if line.contains("failed") {
                line.split_whitespace()
                    .find(|word| word.parse::<u32>().is_ok() && line.split(word).nth(1).unwrap_or("").contains("failed"))
                    .and_then(|s| s.parse::<u32>().ok())
            } else {
                None
            }
        })
        .sum::<u32>();
    
    assert_eq!(failed_count, 0, "Some integration tests failed");
}
