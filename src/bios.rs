use std::io::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use iz80::Machine;
use iz80::Registers;
use super::cpm_machine::*;
use super::constants::*;

pub struct Bios {
    stdin_channel: Receiver<u8>,
    next_char: Option<u8>

}

const BIOS_COMMAND_NAMES: [&'static str; 16] = [
    "BOOT", "WBOOT", "CONST", "CONIN", "CONOUT",
    "LIST", "PUNCH", "READER", "SELDSK", "SETTRK",
    "SETSEC", "SETDMA", "READ", "WRITE", "LISTST",
    "SECTRAN"];

impl Bios {
    pub fn new() -> Bios {
        let (tx, rx) = mpsc::channel::<u8>();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            for mut c in buffer.bytes() {
                if c == 10 {c = 13};
                tx.send(c).unwrap();
            }
        });
        Bios {
            stdin_channel: rx,
            next_char: None
        }
    }

    pub fn setup(&self, machine: &mut CpmMachine) {
        /*
        Setup BIOS location and entry points
        .org $0
            jp BIOS_BASE_ADDRESS + 3
        */
        machine.poke(0, 0xc3 /* jp nnnn */);
        machine.poke16(1, BIOS_BASE_ADDRESS + 3); // Warm start is the second entrypoin in BIOS
        // We put ret on all the addresses
        for i in 0..0x80 { // 0x34 should be enough to cover the 17 entry points.
            machine.poke(BIOS_BASE_ADDRESS + i, 0xc9 /*ret*/);
        }
        // MBASIC assumes this are all JMP xxxx. It copies the destination address
        // and gos there. That's why it calls c9c9
    }

    fn pool_keyboard(&mut self) {
        if self.next_char == None {
            self.next_char = match self.stdin_channel.try_recv() {
                Ok(key) => Some(key),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => panic!("Stdin disconnected")
            }
        }
    }

    pub fn read(&mut self) -> u8 {
        self.pool_keyboard();

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

    pub fn status(&mut self) -> u8 {
        self.pool_keyboard();

        match self.next_char {
            Some(_) => 0xff,
            None => {
                // Avoid 100% CPU usage waiting for input.
                thread::sleep(Duration::from_nanos(100)); 
                0
            }
        }
    }

    pub fn write(&self, ch: u8) {
        print!("{}", ch as char);
        stdout().flush().unwrap();
    }

    pub fn execute(&mut self, reg: &mut Registers, call_trace: bool) -> bool {
        // We fo the BIOS actions outside the emulation.
        let pc = reg.pc();
        if pc >= BIOS_BASE_ADDRESS {
            let offset = pc - BIOS_BASE_ADDRESS;
            if offset < 0x80 && (offset % 3) == 0 {
                /*
                We are on the first byte of the tree reserved for each
                vector. We execute the action and the let the RET run.
                */
                let command = offset / 3;
                if call_trace {
                    let name = if command < BIOS_COMMAND_NAMES.len() as u16 {
                        BIOS_COMMAND_NAMES[command as usize]
                    } else {
                        "unknown"
                    };
                    println!("[[BIOS command {}: {}]]", command, name);
                }
                /*
                See: http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch6.htm#Table_6-5

                0  BOOT: Cold start routine
                1  WBOOT: Warm boot - reload command processor
                2  CONST: Console status
                3  CONIN: Console input
                4  CONOUT: Console output
                5  LIST: Printer output
                6  PUNCH: Paper tape punch output
                7  READER: Paper tape reader input
                8  SELDSK: Select disc drive
                9  SETTRK: Set track number
                10 SETSEC: Set sector number
                11 SETDMA: Set DMA address
                12 READ: Read a sector
                13 WRITE: Write a sector
                14 LISTST: Status of list device
                15 SECTRAN: Sector translation for skewing
                */
                match command {
                    0 => { // BOOT: Cold Start Routine
                        println!("Terminated. cold restart");
                        return true;
                    }
                    1 => { // WBOOT: Warm boot.
                        // Reload command processor. We will go back to the host.
                        println!("Terminated, warm restart");
                        return true;
                    }
                    2 => { // CONST: Check for console ready
                        /*
                        You should sample the status of the currently assigned
                        console device and return 0FFH in register A if a
                        character is ready to read and 00H in register A if no
                        console characters are ready. 
                        */
                        let res8 = self.status();
                        reg.set_a(res8);
                    }
                    3 => { // CONIN: Console Input
                        /*
                        The next console character is read into register A, and
                        the parity bit is set, high-order bit, to zero. If no
                        console character is ready, wait until a character is
                        typed before returning. 
                        */
                        let res8 = self.read();
                        reg.set_a(res8);
                    }
                    _ => {
                        print!("BIOS command {} not implemented.\n", command);
                        panic!("BIOS command not implemented");
                    }    
                }
            }
        }
        false
    }
}
