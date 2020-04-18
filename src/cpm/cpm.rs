extern crate z80;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};

use z80::cpu::Cpu;
use z80::memory_io::*;
use z80::registers::*;
use z80::state::State;

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

fn main() {
    // Parse arguments
    let matches = App::new("Z80 CP/M 2.2 emulator")
        .arg(Arg::with_name("INPUT")
            .help("The z80 image to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("bdos_trace")
            .short("t")
            .long("bdos_trace")
            .help("Trace BDOS calls"))
        .get_matches();
    let filename = matches.value_of("INPUT").unwrap();
    let bdos_trace = matches.is_present("bdos_trace");
    
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

        // We do the BDOS actions outside the emulation just before the RTS
        if state.reg.get_pc() == 0x0007 {
            cpm_console.pool_keyboard();

            let command = state.reg.get8(Reg8::C);
            if bdos_trace {
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
                    if bdos_trace {
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
                    cpm_file.set_dma(state.reg.get16(Reg16::DE));
                },
                33 => { // F_READRAND - Random access read record
                    let fcb = Fcb::new(state.reg.get16(Reg16::DE), &machine);
                    if bdos_trace {
                        print!("[Read record {:x}]", fcb.get_random_record_number());
                    }
                    let res = cpm_file.read_rand(&fcb);
                    if res == 0 {
                        cpm_file.load_buffer(&mut machine);
                    }
                    state.reg.set_a(res);
                }

                _ => {
                    print!("Command {} not implemented.\n", command);
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