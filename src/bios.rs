use iz80::*;

use crate::cpm_machine::*;
use crate::constants::*;
use crate::console_emulator::ConsoleEmulator;
use crate::terminal::TerminalEmulator;


pub struct Bios {
    //console: &'a mut dyn ConsoleEmulator,
    terminal: Box<dyn TerminalEmulator>,
    ctrl_c_count: u8
}

const BIOS_COMMAND_NAMES: [&str; 17] = [
    "BOOT", "WBOOT", "CONST", "CONIN", "CONOUT",
    "LIST", "PUNCH", "READER", "HOME", "SELDSK",
    "SETTRK", "SETSEC", "SETDMA", "READ", "WRITE",
    "LISTST", "SECTRAN"];

const BIOS_ENTRY_POINT_COUNT: usize = 30;
const BIOS_RET_TRAP_START: u16 = BIOS_BASE_ADDRESS + 0x80;

impl Bios {
    pub fn new(/*console: &mut dyn ConsoleEmulator, */terminal: Box<dyn TerminalEmulator>) -> Bios {
        Bios {
            //console,
            terminal,
            ctrl_c_count: 0
        }
    }

    pub fn setup(&self, machine: &mut CpmMachine) {
        // Setup warm start at 0x000
        machine.poke(0, 0xc3 /* jp nnnn */);
        machine.poke16(1, BIOS_BASE_ADDRESS + 3); // Warm start is the second entrypoint in BIOS

        // At BIOS_BASE_ADDRESS we need a "JMP address" for each entry point. At
        // the destination we will put a RET and trap that on the emulator.
        // Programs like MBASIC expect this and copy the address.
        for i in 0..BIOS_ENTRY_POINT_COUNT {
            let entry_point = BIOS_BASE_ADDRESS + (i * 3) as u16;
            let ret_trap = BIOS_RET_TRAP_START + i as u16;
            machine.poke(entry_point, 0xc3 /* jp nnnn */);
            machine.poke16(entry_point+1, ret_trap);
            machine.poke(ret_trap, 0xc9 /*ret*/);
        }
    }

    pub fn status(&mut self, console: &mut dyn ConsoleEmulator) -> u8 {
        if console.status() {
            0xff
        } else {
            0
        }
    }

    pub fn read(&mut self, console: &mut dyn ConsoleEmulator) -> u8 {
        let ch = console.read();
        if ch == 3 { // Control-C
            self.ctrl_c_count += 1;
        } else {
            self.ctrl_c_count = 0;
        }
        ch
    }

    pub fn write(&mut self, console: &mut dyn ConsoleEmulator, ch: u8) {
        let stream = self.terminal.translate(ch);
        console.put(stream);
    }

    pub fn write_string(&mut self, console: &mut dyn ConsoleEmulator, text: &str) {
        for ch in text.chars() {
            self.write(console, ch as u8);
        }
    }


    pub fn stop(&self) -> bool {
        self.ctrl_c_count > 1
    }

    pub fn execute(&mut self, console: &mut dyn ConsoleEmulator, reg: &mut Registers, call_trace: bool) -> ExecutionResult {
        if self.stop() {
            // Stop with two control-c
            self.ctrl_c_count = 0;
            return ExecutionResult::StopConfirm;
        }

        let pc = reg.pc();
        if pc >= BIOS_RET_TRAP_START {
            let command = pc - BIOS_RET_TRAP_START;
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
            8  HOME: Go to track 0
            9  SELDSK: Select disc drive
            10  SETTRK: Set track number
            11 SETSEC: Set sector number
            12 SETDMA: Set DMA address
            13 READ: Read a sector
            14 WRITE: Write a sector
            15 LISTST: Status of list device
            16 SECTRAN: Sector translation for skewing
            */
            match command {
                0 => { // BOOT: Cold Start Routine
                    println!("Terminated. cold restart");
                    return ExecutionResult::ColdBoot;
                }
                1 => { // WBOOT: Warm boot.
                    // Reload command processor. We will go back to the host.
                    return ExecutionResult::WarmBoot;
                }
                2 => { // CONST: Check for console ready
                    // You should sample the status of the currently assigned
                    // console device and return 0FFH in register A if a
                    // character is ready to read and 00H in register A if no
                    // console characters are ready. 
                let res8 = self.status(console);
                    reg.set_a(res8);
                }
                3 => { // CONIN: Console Input
                    // The next console character is read into register A, and
                    // the parity bit is set, high-order bit, to zero. If no
                    // console character is ready, wait until a character is
                    // typed before returning. 
                    let res8 = self.read(console);
                    reg.set_a(res8);
                }
                4 => {  // CONOUT: Console Output
                    // The character is sent from register C to the console
                    // output device. The character is in ASCII, with high-order
                    // parity bit set to zero. You might want to include a
                    // time-out on a line-feed or carriage return, if the
                    // console device requires some time interval at the end of
                    // the line (such as a TI Silent 700 terminal). You can
                    // filter out control characters that cause the console
                    // device to react in a strange way (CTRL-Z causes the Lear-
                    // Siegler terminal to clear the screen, for example). 
                    self.write(console, reg.get8(Reg8::C));
                }
                _ => {
                    eprintln!("BIOS command {} not implemented.\n", command);
                    return ExecutionResult::Stop;
                }    
            }
        }
        ExecutionResult::Continue
    }
}
