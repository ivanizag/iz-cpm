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


//static PROGRAM: &'static [u8] = include_bytes!("rom/zexall.com");
static PROGRAM: &'static [u8] = include_bytes!("rom/TINYBAS.COM");

fn main() {
    let mut machine = CpmMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    // Init console
    let mut stdout = stdout();
    let stdin_channel = spawn_stdin_channel();
    let mut next_char: Option<u8> = None;

    // Load program
    let code = PROGRAM;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x100 + i as u16, code[i]);
    }

    /*
    System call 5

    .org $5
        out ($0), a
        ret
    */
    let code = [0xD3, 0x00, 0xC9];
    for i in 0..code.len() {
        machine.poke(5 + i as u16, code[i]);
    }

    state.reg.set_pc(0x100);
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
        }

        if state.reg.get_pc() == 0x0000 {
            println!("Terminated in address 0x0000");
            break;
        }

        if state.reg.get_pc() == 0x0007 {
            // Keyboard
            if next_char == None {
                next_char = match stdin_channel.try_recv() {
                    Ok(key) => Some(key),
                    Err(TryRecvError::Empty) => None,
                    Err(TryRecvError::Disconnected) => panic!("Stdin disconnected")
                }
            }

            // We do the BDOS actions outside the emulation just before the RTS
            let command = state.reg.get8(Reg8::C); 
            //print!("\n[[BDOS command {}]]", command);
            match command {
                // See https://www.seasip.info/Cpm/bdos.html
                1=> {
                    /*
                    BDOS function 1 (C_READ) - Console input
                    Entered with C=1. Returns A=L=character.

                    Wait for a character from the keyboard; then echo it to
                    the screen and return it.
                    */
                    match next_char {
                        Some(ch) => {
                            state.reg.set8(Reg8::A, ch);
                            state.reg.set8(Reg8::L, ch);
                        },
                        None => {
                            // Blocks waiting for char
                            let ch = stdin_channel.recv().unwrap();
                            state.reg.set8(Reg8::A, ch);
                            state.reg.set8(Reg8::L, ch);
                        }
                    }
                    next_char = None;
                    // No need to echo. The console has done that already.
                }
                2 => {
                    /*
                    BDOS function 2 (C_WRITE) - Console output
                    Entered with C=2, E=ASCII character.

                    Send the character in E to the screen. Tabs are expanded
                    to spaces. Output can be paused with ^S and restarted with
                    ^Q (or any key under versions prior to CP/M 3). While the
                    output is paused, the program can be terminated with ^C.
                    */
                    print!("{}", state.reg.get8(Reg8::E) as char);
                    stdout.flush().unwrap();
                },
                9 => {
                    /*
                    BDOS function 9 (C_WRITESTR) - Output string
                    Entered with C=9, DE=address of string.

                    Display a string of ASCII characters, terminated with the
                    $ character. Thus the string may not contain $ characters
                    - so, for example, the VT52 cursor positioning command ESC Y
                    y+32 x+32 will not be able to use row 4.
                    Under CP/M 3 and above, the terminating character can be
                    changed using BDOS function 110.
                    */
                    let mut address = state.reg.get16(Reg16::DE);
                    let mut msg = String::new();
                    loop {
                        let ch = machine.peek(address) as char;
                        address += 1;
                
                        if ch == '$'{
                            break;
                        }
                        msg.push(ch);
                    }
                    print!("{}", msg);
                    stdout.flush().unwrap();
                },
                11 => {
                    /*
                    BDOS function 11 (C_STAT) - Console status
                    Entered with C=0Bh. Returns A=L=status

                    Returns A=0 if no characters are waiting, nonzero if a
                    character is waiting.
                    */
                    match next_char {
                        Some(_) => {
                            state.reg.set8(Reg8::A, 1);
                            state.reg.set8(Reg8::L, 1);
                        },
                        None => {
                            state.reg.set8(Reg8::A, 0);
                            state.reg.set8(Reg8::L, 0);

                            // Avoid 100% CPU usage waiting for input.
                            thread::sleep(Duration::from_millis(1));  
                        }
                    }
                }
                _ => {
                    print!("Command {:02x} not implemented.\n", command);
                    panic!("BDOS command not implemented");
                }
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

