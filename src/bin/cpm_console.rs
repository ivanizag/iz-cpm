use std::io::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use z80::memory_io::*;
use super::cpm_machine::*;

pub struct CpmConsole {
    stdin_channel: Receiver<u8>,
    next_char: Option<u8>
}

impl CpmConsole {
    pub fn new() -> CpmConsole {
        let (tx, rx) = mpsc::channel::<u8>();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            for mut c in buffer.bytes() {
                if c == 10 {c = 13};
                tx.send(c).unwrap();
            }
        });
        CpmConsole {
            stdin_channel: rx,
            next_char: None
        }
    }

    pub fn pool_keyboard(&mut self) {
        if self.next_char == None {
            self.next_char = match self.stdin_channel.try_recv() {
                Ok(key) => Some(key),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => panic!("Stdin disconnected")
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
                self.stdin_channel.recv().unwrap()
            }
        }
    }

    pub fn write(&self, ch: u8) {
        print!("{}", ch as char);
        stdout().flush().unwrap();
    }

    pub fn write_string(&self, address: u16, machine: &CpmMachine) {
        let mut index = address;
        let mut msg = String::new();
        loop {
            let ch = machine.peek(index) as char;
            index += 1;
    
            if ch == '$'{
                break;
            }
            msg.push(ch);
        }
        print!("{}", msg);
        stdout().flush().unwrap();
    }

    pub fn status(&self) -> u8 {
        match self.next_char {
            Some(_) => 0xff,
            None => {
                // Avoid 100% CPU usage waiting for input.
                thread::sleep(Duration::from_millis(1)); 
                0
            }
        }
    }
}