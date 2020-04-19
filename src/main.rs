use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};

use iz80::*;

mod cpm_console;
mod cpm_drive;
mod cpm_file;
mod cpm_machine;
mod fcb;

use self::cpm_console::*;
use self::cpm_drive::*;
use self::cpm_file::*;
use self::cpm_machine::*;
use self::fcb::*;

const BIOS_BASE_ADDRESS: u16 = 0xfa00;

fn main() {
    // Parse arguments
    let matches = App::new("Z80 CP/M 2.2 emulator")
        .arg(Arg::with_name("INPUT")
            .help("The z80 image to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("call_trace")
            .short("t")
            .long("call-trace")
            .help("Trace BDOS and BIOS calls"))
        .arg(Arg::with_name("cpu_trace")
            .short("c")
            .long("cpu-trace")
            .help("Trace BDOS and BIOS calls"))
        .get_matches();
    let filename = matches.value_of("INPUT").unwrap();
    let call_trace = matches.is_present("call_trace");
    let cpu_trace = matches.is_present("cpu_trace");
    
    // Init system
    let mut machine = CpmMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();
    let mut cpm_console = CpmConsole::new();
    let mut cpm_drive= CpmDrive::new();
    let mut cpm_file = CpmFile::new();

    // Load program
    let mut file = File::open(filename).unwrap();
    let mut buf = [0u8;65536];
    let size = file.read(&mut buf).unwrap();
    for i in 0..size {
        machine.poke(0x100 + i as u16, buf[i]);
    }

    /*
    Setup BIOS location and entry points
    .org $0
        jp BIOS_BASE_ADDRESS + 3
    */
    let warm_start = BIOS_BASE_ADDRESS + 3; // Warm start is the second entrypoin in BIOS*/ 
    machine.poke(0, 0xc3 /* jp nnnn */);
    machine.poke(1, warm_start as u8);
    machine.poke(2, (warm_start >> 8) as u8);
    // We put ret on all the addresses
    for i in 0..0x80 { // 0x34 should be enough to cover the 17 entry points.
        machine.poke(BIOS_BASE_ADDRESS + i, 0xc9 /*ret*/);
    }

    /*
    Setup BDOS: System call 5
    .org $5
        out ($0), a
        ret
    */
    let code = [0xD3, 0x00, 0xC9];
    for i in 0..code.len() {
        machine.poke(5 + i as u16, code[i]);
    }



    state.reg.set_pc(0x100);
    cpu.set_trace(cpu_trace);
    loop {
        cpu.execute_instruction(&mut state, &mut machine);

        let pc = state.reg.pc();
        // We fo the BIOS actions outside the emulation.
        if pc >= BIOS_BASE_ADDRESS {
            let offset = pc - BIOS_BASE_ADDRESS;
            if offset < 0x80 && (offset % 3) == 0 {
                /*
                We are on the first byte of the tree reserved for each
                vector. We execute the action and the let the RET run.
                */
                let command = offset / 3;
                if call_trace {
                    print!("\n[[BIOS function {}]]", command);
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
                        break;                                }
                    1 => { // WBOOT: Warm boot.
                        // Reload command processor. We will go back to the host.
                        println!("Terminated, warm restart");
                        break;                                }
                    _ => {
                        print!("BIOS command {} not implemented.\n", command);
                        panic!("BIOS command not implemented");
                    }    
                }
            }
        }

        // We do the BDOS actions outside the emulation just before the RTS
        if pc == 0x0007 {
            cpm_console.pool_keyboard();

            let command = state.reg.get8(Reg8::C);
            if call_trace /*&& command > 11*/ {
                print!("\n[[BDOS command {}]]", command);
            }

            match command {
                // See https://www.seasip.info/Cpm/bdos.html
                // See http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch5.htm
                1=> { // C_READ - Console input
                    state.reg.set_a(cpm_console.read())
                }
                2 => { // C_WRITE - Console output
                    cpm_console.write(state.reg.get8(Reg8::E));
                },
                6 => { // C_RAWIO - Direct console I/O
                    state.reg.set_a(cpm_console.raw_io(state.reg.get8(Reg8::E)))
                }
                9 => { // C_WRITESTR - Output string
                    let address = state.reg.get16(Reg16::DE);
                    cpm_console.write_string(address, &machine);
                },
                11 => { // C_STAT - Console status
                    state.reg.set_a(cpm_console.status());
                },
                12 => { // S_BDOSVER - Return version number
                    state.reg.set16(Reg16::HL, get_version());
                },
                13 => { // DRV_ALLRESET - Reset disk system
                    cpm_drive.reset();
                    cpm_file.reset();
                },
                14 => { // DRV_SET - Select disk
                    let selected = state.reg.get8(Reg8::E);
                    cpm_drive.select(selected);
                },
                15 => { // F_OPEN - Open file
                    let fcb = Fcb::new(state.reg.get16(Reg16::DE), &machine);
                    if call_trace {
                        print!("[[Open file {}]]", fcb.get_name());
                    }
                    let res = cpm_file.open(&fcb);
                    state.reg.set_a(res);
                },
                16 => { // F_CLOSE - Close file
                    let fcb = Fcb::new(state.reg.get16(Reg16::DE), &machine);
                    let res = cpm_file.close(&fcb);
                    state.reg.set_a(res);
                },
                220 /*20*/ => { // F_READ - read next record
                    /*
                    Given that the FCB addressed by DE has been activated through an
                    Open or Make function, the Read Sequential function reads the
                    next 128-byte record from the file into memory at the current DMA
                    address. The record is read from position cr of the extent, and
                    the cr field is automatically incremented to the next record
                    position. If the cr field overflows, the next logical extent is
                    automatically opened and the cr field is reset to zero in
                    preparation for the next read operation. The value 00H is returned
                    in the A register if the read operation was successful, while a
                    nonzero value is returned if no data exist at the next record
                    position (for example, end-of-file occurs). 
                    */
                    //let res = cpm_file.read(mem, Reg16::DE);
                    //state.reg.set_a(res);
                    //TODO
                    state.reg.set_a(0xff);
                },
                24 => { // DRV_LOGINVEC - Return Log-in Vector
                    let vector = cpm_drive.get_log_in_vector();
                    state.reg.set16(Reg16::HL, vector);
                    state.reg.set_a(vector as u8);
                },
                25 => { // DRV_GET - Return current disk
                    state.reg.set_a(cpm_drive.get_current());
                },
                26 => { // F_DMAOFF - Set DMA address
                    let dma = state.reg.get16(Reg16::DE);
                    if call_trace {
                        print!("[Set dma {:04x}]", dma);
                    }
                    cpm_file.set_dma(dma);
                },
                33 => { // F_READRAND - Random access read record
                    let fcb = Fcb::new(state.reg.get16(Reg16::DE), &machine);
                    if call_trace {
                        print!("[Read record {:x} into {:04x}]",
                            fcb.get_random_record_number(), cpm_file.get_dma());
                    }
                    let res = cpm_file.read_rand(&fcb);
                    if res == 0 {
                        cpm_file.load_buffer(&mut machine);
                    }
                    state.reg.set_a(res);
                }

                _ => {
                    print!("BDOS command {} not implemented.\n", command);
                    panic!("BDOS command not implemented");
                }
            }
        }
    }
}

fn get_version() -> u16 {
    /*
    Function 12 provides information that allows version independent
    programming. A two-byte value is returned, with H = 00
    designating the CP/M release (H = 01 for MP/M) and L = 00 for all
    releases previous to 2.0. CP/M 2.0 returns a hexadecimal 20 in
    register L, with subsequent version 2 releases in the hexadecimal
    range 21, 22, through 2F. Using Function 12, for example, the
    user can write application programs that provide both sequential
    and random access functions. 
    */
    0x0022 // CP/M 2.2 for Z80
}