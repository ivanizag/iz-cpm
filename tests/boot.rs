mod common;
use common::*;
use izcpm::Step;


#[test]
fn test_boot() {
    run_script(vec!(
        Step::Expect("A>"),
    ));
}

#[test]
fn test_change_drive() {
    run_script(vec!(
        Step::Expect("A>"),
        Step::Input("B:\r"),
        Step::Expect("B>"),
    ));
}

