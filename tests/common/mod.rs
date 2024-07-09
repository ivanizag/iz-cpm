
use izcpm::ConsoleTest;
pub use izcpm::Step as Step;

#[allow(dead_code)]
pub fn run_script(script: Vec<Step>) {
    let mut console = ConsoleTest::new(script);
    izcpm::run(None, &mut console);
}

#[allow(dead_code)]
pub fn run_script_with_args(script: Vec<Step>, args: Vec<&str>) {
    let mut console = ConsoleTest::new(script);
    izcpm::run(Some(args), &mut console);
}
