mod common;

mod unit_test {
    mod test_time_action;
}

#[test]
fn run_all_common_unit_test() {
    common::run_common_tests("unit_test");
}
