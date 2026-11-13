mod common;
use common::*;
use izcpm::Step;

// Regression tests for https://github.com/ivanizag/iz-cpm/issues/30 and
// https://github.com/ivanizag/iz-cpm/issues/31
// Addresses wrap around at 0xFFFF like in a real Z80, they must not panic.

#[test]
fn test_fcb_wraps_around_end_of_ram() {
    run_script_with_args(vec!(
        Step::Expect("A>"),
        Step::Input("WRAPFCB\r"),
        Step::Expect("A>"),
        ), vec!("-a", "tests/artifacts")
    );
}

#[test]
fn test_dma_wraps_around_end_of_ram() {
    run_script_with_args(vec!(
        Step::Expect("A>"),
        Step::Input("WRAPDMA\r"),
        Step::Expect("A>"),
        ), vec!("-a", "tests/artifacts")
    );
}
