mod common;

mod integration {
    mod time_output_test;
}

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration");
}
