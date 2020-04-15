
/*
http://cpuville.com/Code/CPM-on-a-new-computer.html
http://cpuville.com/Code/Tiny-BASIC.html
*/

extern crate z80;

use std::io::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use z80::cpu::Cpu;
use z80::memory_io::*;
use z80::registers::*;
use z80::state::State;

static TINY_BASIC: &'static [u8] = include_bytes!("rom/tinybasic2dms.bin");
//static MONITOR: &'static [u8] = include_bytes!("rom/2K_ROM_8.bin");


fn main() {
    let mut machine = CpmMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    // Init console
    let mut stdout = stdout();
    let stdin_channel = spawn_stdin_channel();
    let mut in_char_waiting = false;

    // Load program
    let code = TINY_BASIC;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x00 + i as u16, code[i]);
    }

    // Init
    state.reg.set_pc(0x00);
    machine.in_values[3] = 1; // TX Ready

    let trace = false;
    cpu.trace = trace;
    loop {
        cpu.execute_instruction(&mut state, &mut machine);

        if trace {
            // CPU registers
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                state.reg.get_pc(),
                state.reg.get16(Reg16::AF),
                state.reg.get16(Reg16::BC),
                state.reg.get16(Reg16::DE),
                state.reg.get16(Reg16::HL),
                state.reg.get16(Reg16::SP),
                state.reg.get16(Reg16::IX),
                state.reg.get16(Reg16::IY),
                state.reg.get8(Reg8::F)
            );

            // Test state
            let addr = 0x1d80 as u16;
            print!("Cpm state 0x{:04x}: ", addr);
            for i in 0..0x10 {
                print!("{:02x} ", machine.peek(addr + i));
            }
            println!("");
        }

        if state.reg.get_pc() == 0x0000 {
            println!("");
            break;
        }

        
        if machine.out_called {
            match machine.out_port {
                2 => {
                    print!("{}", machine.out_value as char);
                    stdout.flush().unwrap();
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.out_called = false;
        }

        if machine.in_called {
            match machine.in_port {
                2 => {
                    in_char_waiting = false;
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.in_called = false;

            // Avoid 100% CPU usage waiting for input.
            thread::sleep(Duration::from_millis(1));  
        }

        if !in_char_waiting {
            // Let's get another char if available
            match stdin_channel.try_recv() {
                Ok(key) => {
                    machine.in_values[2] = key;
                    in_char_waiting = true;
                    machine.in_values[3] = 3; // RX Ready
                },
                Err(TryRecvError::Empty) => {
                    machine.in_values[3] = 1; // RX Not ready
                },
                Err(TryRecvError::Disconnected) => {},
            }
        }
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        for mut c in buffer.bytes() {
            if c == 10 {c = 13};
            tx.send(c).unwrap();
        }
    });
    rx
}

struct CpmMachine {
    mem: [u8; PLAIN_MEMORY_SIZE],
    in_values: [u8; 256],
    in_called: bool,
    in_port: u8,
    out_called: bool,
    out_port: u8,
    out_value: u8
}

impl CpmMachine {
    pub fn new() -> CpmMachine {
        CpmMachine {
            mem: [0; PLAIN_MEMORY_SIZE],
            in_values: [0; 256],
            out_called: false,
            out_port: 0,
            out_value: 0,
            in_called: false,
            in_port: 0
        }
    }
}

impl Machine for CpmMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let value = self.in_values[address as u8 as usize];
        if value != 1 {
            //print!("Port {:04x} in {:02x}\n", address, value);
        }
        self.in_port = address as u8;
        self.in_called = true;
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        //print!("Port {:04x} out {:02x} {}\n", address, value, value as char);
        self.out_port = address as u8;
        self.out_value = value;
        self.out_called = true;
    }
}


