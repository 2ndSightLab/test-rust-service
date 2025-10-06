mod common;

mod integration {
    mod test_service_binary_exists;
    mod test_time_interval_looping;
}

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration");
}
