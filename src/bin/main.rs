use izcpm::{run, Console};

fn main() {
    let mut console = Console::new();
    let exit_code = run(None, &mut console);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}