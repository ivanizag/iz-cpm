use std::io::{Read, stdin, Write, stdout};
use std::thread;
use std::time::Duration;

use termios::*;

use super::translate::Adm3aToAnsi;

const STDIN_FD: i32 = 0;

pub struct Console {
    initial_termios: Option<Termios>,
    next_char: Option<u8>,
    translator: Adm3aToAnsi
}

impl Console {
    pub fn new() -> Console {
        // Prepare terminal
        let initial_termios = Termios::from_fd(STDIN_FD).ok();

        let c = Console {
            initial_termios: initial_termios,
            next_char: None,
            translator: Adm3aToAnsi::new(),
        };

        c.setup_host_terminal(false);
        c
    }

    fn setup_host_terminal(&self, blocking: bool) {
        if let Some(initial) = self.initial_termios {
            let mut new_term = initial.clone();
            new_term.c_iflag &= !(IXON | ICRNL);
            new_term.c_lflag &= !(ISIG | ECHO | ICANON | IEXTEN);
            new_term.c_cc[VMIN] = if blocking {1} else {0};
            new_term.c_cc[VTIME] = 0;
            tcsetattr(STDIN_FD, TCSANOW, &new_term).unwrap();
        }
    }

    pub fn status(&mut self) -> bool {
        match self.next_char {
            Some(_) => true,
            None => {
                let mut buf = [0];
                let size = stdin().read(&mut buf).unwrap_or(0);
                if size != 0 {
                    self.next_char = Some(buf[0]);
                    true
                } else {
                    // Avoid 100% CPU usage waiting for input.
                    thread::sleep(Duration::from_nanos(100));
                    false
                }
            }
        }
    }

    pub fn read(&mut self) -> u8 {
        match self.next_char {
            Some(ch) => {
                self.next_char = None;
                ch
            },
            None => {
                // Blocks waiting for char
                self.setup_host_terminal(true);
                let mut buf = [0];
                stdin().read(&mut buf).unwrap();
                self.setup_host_terminal(false);
                buf[0]
            }
        }
    }

    pub fn put(&mut self, ch: u8) {
        if let Some(sequence) = self.translator.translate(ch) {
            print!("{}", sequence);
            stdout().flush().unwrap();
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        if let Some(initial) = self.initial_termios {
            tcsetattr(STDIN_FD, TCSANOW, &initial).unwrap();
        }
    }
}
