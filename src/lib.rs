mod bdos;
mod bios;
mod constants;
mod bdos_console;
mod bdos_drive;
mod bdos_environment;
mod bdos_file;
mod console_emulator;
mod console_test;
mod cpm_machine;
mod fcb;
mod terminal;
mod terminal_adm3a;
mod run;

#[cfg(windows)]
mod console_windows;
#[cfg(unix)]
mod console_unix;

pub use run::run as run;
#[cfg(windows)]
pub use console_windows::Console as Console;
#[cfg(unix)]
pub use console_unix::Console as Console;

pub use console_test::ConsoleTest as ConsoleTest;
pub use console_test::Step as Step;
