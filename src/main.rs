use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::thread;
use std::time::Duration;


use clap::{Arg, App};
#[macro_use(defer)] extern crate scopeguard;

use iz80::*;

mod bdos;
mod bios;
mod constants;
mod bdos_console;
mod bdos_drive;
mod bdos_environment;
mod bdos_file;
mod cpm_machine;
mod fcb;
mod terminal;

use self::bdos::Bdos;
use self::bios::Bios;
use self::constants::*;
use self::cpm_machine::*;
use self::fcb::*;

// Welcome message 1970's style
const WELCOME: &'static str =
"iz-cpm https://github.com/ivanizag/iz-cpm
CP/M 2.2 Copyright (c) 1979 by Digital Research
Press ctrl-c ctrl-c to return to host";

static CCP_BINARY: &'static [u8] = include_bytes!("../cpm22/OS2CCP.BIN");

fn main() {
    // Parse arguments
    let matches = App::new(WELCOME)
        .arg(Arg::with_name("CMD")
            .help("The z80 image to run")
            .required(false)
            .index(1))
            .arg(Arg::with_name("ARGS")
            .help("Parameters for the given command")
            .required(false)
            .index(2))
        .arg(Arg::with_name("call_trace")
            .short("t")
            .long("call-trace")
            .help("Trace BDOS and BIOS calls"))
        .arg(Arg::with_name("call_trace_all")
            .short("T")
            .long("call-trace-all")
            .help("Trace BDOS and BIOS calls excluding screen I/O"))
        .arg(Arg::with_name("cpu_trace")
            .short("z")
            .long("cpu-trace")
            .help("Trace Z80 instructions execution"))
        .arg(Arg::with_name("slow")
            .short("s")
            .long("slow")
            .help("Run slower"))
        .arg(Arg::with_name("disk_a").long("disk-a").value_name("path").short("a").default_value(".").help("directory to map disk A:"))
        .arg(Arg::with_name("disk_b").long("disk-b").value_name("path").short("b").help("directory to map disk B:"))
        .arg(Arg::with_name("disk_c").long("disk-c").value_name("path").short("c").help("directory to map disk C:"))
        .arg(Arg::with_name("disk_d").long("disk-d").value_name("path").short("d").help("directory to map disk D:"))
        .arg(Arg::with_name("disk_e").long("disk-e").value_name("path").help("directory to map disk E:"))
        .arg(Arg::with_name("disk_f").long("disk-f").value_name("path").help("directory to map disk F:"))
        .arg(Arg::with_name("disk_g").long("disk-g").value_name("path").help("directory to map disk G:"))
        .arg(Arg::with_name("disk_h").long("disk-h").value_name("path").help("directory to map disk H:"))
        .arg(Arg::with_name("disk_i").long("disk-i").value_name("path").help("directory to map disk I:"))
        .arg(Arg::with_name("disk_j").long("disk-j").value_name("path").help("directory to map disk J:"))
        .arg(Arg::with_name("disk_k").long("disk-k").value_name("path").help("directory to map disk K:"))
        .arg(Arg::with_name("disk_l").long("disk-l").value_name("path").help("directory to map disk L:"))
        .arg(Arg::with_name("disk_m").long("disk-m").value_name("path").help("directory to map disk M:"))
        .arg(Arg::with_name("disk_n").long("disk-n").value_name("path").help("directory to map disk N:"))
        .arg(Arg::with_name("disk_o").long("disk-o").value_name("path").help("directory to map disk O:"))
        .arg(Arg::with_name("disk_p").long("disk-p").value_name("path").help("directory to map disk P:"))
        .get_matches();
    let filename = matches.value_of("CMD");
    let params = matches.value_of("ARGS");
    let cpu_trace = matches.is_present("cpu_trace");
    let call_trace = matches.is_present("call_trace") || matches.is_present("call_trace_all");
    let call_trace_skip_console = !matches.is_present("call_trace_all");
    let slow = matches.is_present("slow");

    // Init device
    let mut machine = CpmMachine::new();
    let mut cpu = Cpu::new();

    // Init cpm
    let mut bios = Bios::new();
    bios.setup(&mut machine);
    let mut bdos = Bdos::new();
    bdos.setup(&mut machine);

    // Assign drives
    for i in 0..15 {
        let res = matches.value_of(format!("disk_{}", (i + 'a' as u8) as char));
        if let Some(path) = res {
            if let Err(err) = fs::read_dir(path) {
                eprintln!("Error with directory \"{}\": {}", path, err);
                process::exit(1);
            }
            bdos.assign_drive(i, path.to_string());
        }
    }

    // Load CCP or program
    let binary: &[u8];
    let binary_address: u16;
    let binary_size: usize;
    let mut buf = [0u8;65536 - (TPA_BASE_ADDRESS as usize)];
    match filename {
        None => {
            // Load TPA
            binary = CCP_BINARY;
            binary_address = CCP_BASE_ADDRESS;
            binary_size = CCP_BINARY.len();
            println!("{}", WELCOME);
        },
        Some(name) => {
            /*
            If the file is found, it is assumed to be a memory image of a
            program that executes in the TPA and thus implicity originates
            at TBASE in memory.
            */
            match File::open(name) {
                Err(err) => {
                    eprintln!("Error opening \"{}\": {}", name, err);
                    process::exit(1);
                },
                Ok(mut file) => {
                    match file.read(&mut buf) {
                        Err(err) => {
                            eprintln!("Error loading \"{}\": {}", name, err);
                            process::exit(1);
                        },
                        Ok(size) => {
                            binary = &buf;
                            binary_address = TPA_BASE_ADDRESS;
                            binary_size = size;
                        }
                    };
                }
            }
        }
    }

    // Load the code in Z80 memory
    for i in 0..binary_size {
        machine.poke(binary_address + i as u16, binary[i]);
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

            /*
            As a convenience, the CCP takes the first two parameters that appear
            in the command tail, attempts to parse them as though they were file
            names, and places the results in FCBI and FCB2. The results, in this
            context, mean that the logical disk letter is converted to its FCB
            representation, and the file name and type, converted to uppercase,
            are placed in the FCB in the correct bytes.
            In addition, any use of "*" in the file name is expanded to one or
            more question marks. For example, a file name of "abc*.*" will be
            converted to a name of "ABC!!???" and type of "???".
            Notice that FCB2 starts only 16 bytes above FCBI, yet a normal FCB
            is at least 33 bytes long (36 bytes if you want to use random access).
            In many cases, programs only require a single file name. Therefore,
            you can proceed to use FCBI straight away, not caring that FCB2 will
            be overwritten.
            */
            let mut parts = p.split_ascii_whitespace();
            if let Some(arg1) = parts.next() {
                if let Some(file1) = name_to_8_3(arg1) {
                    Fcb::new(FCB1_ADDRESS).set_name_direct(&mut machine, file1);
                }
            }
            if let Some(arg2) = parts.next() {
                if let Some(file2) = name_to_8_3(arg2) {
                    Fcb::new(FCB2_ADDRESS).set_name_direct(&mut machine, file2);
                }
            }
        }
    }

    /*
    Upon entry to a transient program, the CCP leaves the stack pointer set to
    an eight-level stack area with the CCP return address pushed onto the stack,
    leaving seven levels before overflow occurs. 
    */
    if binary_address == TPA_BASE_ADDRESS {
        let mut sp = TPA_STACK_ADDRESS;
        // Push 0x0000
        machine.poke(sp, (0x0000 >> 8) as u8);
        sp -= 1;
        machine.poke(sp, 0x0000 as u8);
        sp -= 1;
        cpu.registers().set16(Reg16::SP, sp);
    }

    // Prepare terminal
    let initial_terminal = bios.initial_terminal();
    bios.setup_host_terminal(false);
    defer! {
        bios::restore_host_terminal(&initial_terminal);
    }

    // Run the emulation
    cpu.registers().set_pc(binary_address);
    cpu.set_trace(cpu_trace);
    let mut n = 0;
    loop {
        cpu.execute_instruction(&mut machine);

        let pc = cpu.registers().pc();

        if cpu.is_halted() {
            println!("HALT instruction");
            break;
        }

        if bios.execute(cpu.registers(), call_trace) {
            println!("Terminated");
            break;
        }

        if bdos.execute(&mut bios, &mut machine, cpu.registers(), call_trace, call_trace_skip_console) {
            break;
        }

        if pc == RESTART_ADDRESS {
            println!("Terminated by JMP 0000h");
            break;
        }

        if pc == BDOS_BASE_ADDRESS - 1 {
            // Guard to detect code reaching BDOS (usually NOPs)
            println!("Executing into BDOS area");
            break;
        }

        if slow {
            n += 1;
            if n > 20 {
                thread::sleep(Duration::from_nanos(1000));
                n = 0;
            }
        }
    }
}
