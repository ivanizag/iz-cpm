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

const SYSTEM_PARAMS_ADDRESS: u16 = 0x0080;
const TPA_BASE_ADDRESS:      u16 = 0x0100;
//const CCP_BASE_ADDRESS:      u16 = 0xf000;
const TPA_STACK_ADDRESS:     u16 = 0xf080; // 16 bytes for an 8 level stack
const BIOS_BASE_ADDRESS:     u16 = 0xff00;
const BDOS_BASE_ADDRESS:     u16 = 0xfe80;

const BIOS_COMMAND_NAMES: [&'static str; 16] = [
    "BOOT", "WBOOT", "CONST", "CONIN", "CONOUT",
    "LIST", "PUNCH", "READER", "SELDSK", "SETTRK",
    "SETSEC", "SETDMA", "READ", "WRITE", "LISTST",
    "SECTRAN"];

const BDOS_COMMAND_NAMES: [&'static str; 38] = [
    "P_TERMCPM", "C_READ", "C_WRITE", "A_READ", "A_WRITE",
    "L_WRITE", "C_RAWIO", "A_STATIN", "A_STATOUT", "C_WRITESTR",
    "C_READSTR", "C_STAT", "S_BDOSVER", "DRV_ALLRESET", "DRV_SET",
    "F_OPEN", "F_CLOSE", "F_SFIRST", "F_SNEXT", "F_DELETE",
    "F_READ", "F_WRITE", "F_MAKE", "F_RENAME", "DRV_LOGINVEC",
    "DRV_GET", "F_DMAOFF", "DRV_ALLOCVEC", "DRV_SETRO", "DRV_ROVEC",
    "F_ATTRIB", "DRV_DPB", "F_USERNUM", "F_READRAND", "F_WRITERAND",
    "F_SIZE", "F_RANDREC", "DRV_RESET"]; 



fn main() {
    // Parse arguments
    let matches = App::new("Z80 CP/M 2.2 emulator")
        .arg(Arg::with_name("CMD")
            .help("The z80 image to run")
            .required(true)
            .index(1))
            .arg(Arg::with_name("ARGS")
            .help("Parameters for the given command")
            .required(false)
            .index(2))
        .arg(Arg::with_name("call_trace")
            .short("t")
            .long("call-trace")
            .help("Trace BDOS and BIOS calls"))
        .arg(Arg::with_name("cpu_trace")
            .short("c")
            .long("cpu-trace")
            .help("Trace BDOS and BIOS calls"))
        .get_matches();
    let filename = matches.value_of("CMD").unwrap();
    let params = matches.value_of("ARGS");
    let call_trace = matches.is_present("call_trace");
    let cpu_trace = matches.is_present("cpu_trace");
    let call_trace_skip_console = true;

    if let Some(p) = params {
        println!("Paramenters: <{}>", p);
    }
    
    // Init device
    let mut machine = CpmMachine::new();
    let mut cpu = Cpu::new();

    // Init cpm
    let mut cpm_console = CpmConsole::new();
    let mut cpm_drive= CpmDrive::new();
    let mut cpm_file = CpmFile::new();

    // Load program
    /*
    If the file is found, it is assumed to be a memory image of a program that
    executes in the TPA and thus implicity originates at TBASE in memory. The
    CCP loads the COM file from the disk into memory starting at TBASE and can
    extend up to CBASE. 
    */
    let mut file = File::open(filename).unwrap();
    let mut buf = [0u8;65536 - (TPA_BASE_ADDRESS as usize)];
    let size = file.read(&mut buf).unwrap();
    for i in 0..size {
        machine.poke(TPA_BASE_ADDRESS + i as u16, buf[i]);
    }

    // Copy parameters
    /*
    As an added convenience, the default buffer area at location BOOT+0080H is
    initialized to the command line tail typed by the operator following the
    program name. The first position contains the number of characters, with
    the characters themselves following the character count. The characters are
    translated to upper-case ASCII with uninitialized memory following the last
    valid character. 
    */
    match params {
        None => machine.poke(SYSTEM_PARAMS_ADDRESS, 0),
        Some(p) => {
            let mut len = p.len();
            if len > 0x7E {
                len = 0x7E; // Max 0x7E chars for parameters
            }
            machine.poke(SYSTEM_PARAMS_ADDRESS, (len + 1) as u8);
            machine.poke(SYSTEM_PARAMS_ADDRESS, ' ' as u8);
            let p_bytes = p.as_bytes();
            for i in 0..len {
                machine.poke(SYSTEM_PARAMS_ADDRESS + (i as u16) + 2, p_bytes[i]);
            }
        }
    }
    /*
    Upon entry to a transient program, the CCP leaves the stack pointer set to
    an eight-level stack area with the CCP return address pushed onto the stack,
    leaving seven levels before overflow occurs. 
    */
    let mut sp = TPA_STACK_ADDRESS;
    // Push 0x0000
    machine.poke(sp, (0x0000 >> 8) as u8);
    sp -= 1;
    machine.poke(sp, 0x0000 as u8);
    sp -= 1;
    cpu.registers().set16(Reg16::SP, sp);

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

    /*
    Setup BDOS location and entry point
    .org $5
        jp BDOS_BASE_ADDRESS
    */
    machine.poke(5, 0xc3 /* jp nnnn */);
    machine.poke16(6, BDOS_BASE_ADDRESS);
    // We put ret on that address
    machine.poke(BDOS_BASE_ADDRESS, 0xc9 /*ret*/);

    cpu.registers().set_pc(0x100);
    cpu.set_trace(cpu_trace);
    loop {
        cpu.execute_instruction(&mut machine);

        let pc = cpu.registers().pc();
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
                    let name = if command < BIOS_COMMAND_NAMES.len() as u16 {
                        BIOS_COMMAND_NAMES[command as usize]
                    } else {
                        "unknown"
                    };
                    print!("\n[[BIOS command {}: {}]]", command, name);
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
                        break;
                    }
                    1 => { // WBOOT: Warm boot.
                        // Reload command processor. We will go back to the host.
                        println!("Terminated, warm restart");
                        break;
                    }
                    2 => { // CONST: Check for console ready
                        /*
                        You should sample the status of the currently assigned
                        console device and return 0FFH in register A if a
                        character is ready to read and 00H in register A if no
                        console characters are ready. 
                        */
                        let res8 = cpm_console.status();
                        cpu.registers().set_a(res8);
                    }
                    3 => { // CONIN: Console Input
                        /*
                        The next console character is read into register A, and
                        the parity bit is set, high-order bit, to zero. If no
                        console character is ready, wait until a character is
                        typed before returning. 
                        */
                        let res8 = cpm_console.read();
                        cpu.registers().set_a(res8);
                    }
                    10 => { // SETSEC: Set sector number
                        let sector = cpu.registers().get8(Reg8::C);
                        if call_trace {
                            println!("Set sector: {}", sector);
                        }
                    }
                    _ => {
                        print!("BIOS command {} not implemented.\n", command);
                        panic!("BIOS command not implemented");
                    }    
                }
            }
        }

        if pc == BDOS_BASE_ADDRESS - 1 {
            // Guard to detect code reaching BDOS (usually NOPs)
            panic!("Executing into BDOS area");

        }

        if pc == BDOS_BASE_ADDRESS {
            cpm_console.pool_keyboard();

            let arg8 = cpu.registers().get8(Reg8::E);
            let arg16 = cpu.registers().get16(Reg16::DE);

            let command = cpu.registers().get8(Reg8::C);
            let bdos_trace = call_trace && !(call_trace_skip_console && command <= 12);
            if bdos_trace {
                let name = if command < BDOS_COMMAND_NAMES.len() as u8 {
                    BDOS_COMMAND_NAMES[command as usize]
                } else {
                    "unknown"
                };
                print!("\n[[BDOS command {}: {}({:04x})]]", command, name, arg16);
            }

            let mut res8: Option<u8> = None;
            let mut res16: Option<u16> = None;

            match command {
                // See https://www.seasip.info/Cpm/bdos.html
                // See http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch5.htm
                1=> { // C_READ - Console input
                    res8 = Some(cpm_console.read())
                }
                2 => { // C_WRITE - Console output
                    cpm_console.write(arg8);
                },
                6 => { // C_RAWIO - Direct console I/O
                    res8 = Some(cpm_console.raw_io(arg8))
                }
                9 => { // C_WRITESTR - Output string
                    cpm_console.write_string(arg16, &machine);
                },
                10 => { // C_READSTR
                    cpm_console.read_string(arg16, &mut machine);
                },
                11 => { // C_STAT - Console status
                    res8 = Some(cpm_console.status());
                },
                12 => { // S_BDOSVER - Return version number
                    res16 = Some(get_version());
                },
                13 => { // DRV_ALLRESET - Reset disk system
                    cpm_drive.reset();
                    cpm_file.reset();
                },
                14 => { // DRV_SET - Select disk
                    cpm_drive.select(arg8);
                },
                15 => { // F_OPEN - Open file
                    let mut fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[[Open file {}]]", fcb.get_name());
                    }
                    res8 = Some(cpm_file.open(&mut fcb));
                },
                16 => { // F_CLOSE - Close file
                    let fcb = Fcb::new(arg16, &mut machine);
                    res8 = Some(cpm_file.close(&fcb));
                },
                19 => { // F_DELETE - Delete file
                    let fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[[Delete file {}]]", fcb.get_name());
                    }
                    res8 = Some(cpm_file.delete(&fcb));
                }
                20 => { // F_READ - Read next record
                    let mut fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[Read record {:x} into {:04x}]",
                            fcb.get_sequential_record_number(), cpm_file.get_dma());
                    }
                    let res = cpm_file.read(&mut fcb);
                    if res == 0 {
                        cpm_file.load_buffer(&mut machine);
                    }
                    res8 = Some(res);
                },
                21 => { // F_WRITE - Write next record
                    cpm_file.save_buffer(&mut machine);
                    let mut fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[Write record {:x} from {:04x}]",
                            fcb.get_sequential_record_number(), cpm_file.get_dma());
                    }
                    let res = cpm_file.write(&mut fcb);
                    res8 = Some(res);
                }
                22 => { // F_MAKE - Create file
                    let mut fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[[Create file {}]]", fcb.get_name());
                    }
                    res8 = Some(cpm_file.make(&mut fcb));
                }
                24 => { // DRV_LOGINVEC - Return Log-in Vector
                    res16 = Some(cpm_drive.get_log_in_vector());
                },
                25 => { // DRV_GET - Return current disk
                    res8 = Some(cpm_drive.get_current());
                },
                26 => { // F_DMAOFF - Set DMA address
                    cpm_file.set_dma(arg16);
                },
                32 => { // F_USERNUM - Get/set user number
                    res8 = Some(cpm_file.get_set_user_number(arg8));
                }
                33 => { // F_READRAND - Random access read record
                    let fcb = Fcb::new(arg16, &mut machine);
                    if call_trace {
                        print!("[Read random record {:x} into {:04x}]",
                            fcb.get_random_record_number(), cpm_file.get_dma());
                    }
                    let res = cpm_file.read_rand(&fcb);
                    if res == 0 {
                        cpm_file.load_buffer(&mut machine);
                    }
                    res8 = Some(res);
                }

                _ => {
                    print!("BDOS command {} not implemented.\n", command);
                    panic!("BDOS command not implemented");
                }
            }

            /*
            Single byte values are returned in register A, with double
            byte values returned in HL, a zero value is returned when the
            function number is out of range. For reasons of compatibility,
            register A = L and register B = H upon return in all cases.
            */
            if let Some(a) = res8 {
                cpu.registers().set8(Reg8::A, a);
                cpu.registers().set8(Reg8::L, a);
                if bdos_trace {
                    print!("[[=>{:02x}]]", a);
                }
            } else if let Some(hl) = res16 {
                cpu.registers().set16(Reg16::HL, hl);
                cpu.registers().set8(Reg8::A, hl as u8);
                cpu.registers().set8(Reg8::B, (hl>>8) as u8);
                if bdos_trace {
                    print!("[[=>{:02x}]]", hl);
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