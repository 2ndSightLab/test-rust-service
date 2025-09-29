mod common;

mod unit_tests {
    mod time_action_test;
}

#[test]
fn run_all_common_unit_tests() {
    common::run_common_tests("unit_tests");
}
