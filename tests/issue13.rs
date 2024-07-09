mod common;
use common::*;
use izcpm::Step;

// Integration tests for issue https://github.com/ivanizag/iz-cpm/issues/13

fn test_issue13(mode: u8) {
    run_script_with_args(vec!(
        Step::Expect("A>"),
        Step::Input("B:\r"),
        Step::Expect("B>"),
        Step::Input(&format!("ret {}\r", mode)),
        Step::Expect("B>"),
        ), vec!("-b", "tests/artifacts")
    );
}


#[test]
fn test_issue13_jp() {
    test_issue13(1);
}

#[test]
fn test_issue13_rst() {
    test_issue13(2);
}

#[test]
fn test_issue13_ret() {
    test_issue13(3);
}

#[test]
fn test_issue13_call() {
    test_issue13(4);
}
