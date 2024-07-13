mod common;
use common::*;
use izcpm::Step;

// Integration tests for issue https://github.com/ivanizag/iz-cpm/issues/16
#[test]
fn test_issue16() {
    run_script_with_args(vec!(
        Step::Expect("A>"),
        Step::Input("SLASH DIR;XYZ\r"),
        Step::Expect("A$DIR"),
        Step::Expect("A$XYZ"),
        Step::Expect("A>"),
        ), vec!("-a", "tests/artifacts")
    );
}
