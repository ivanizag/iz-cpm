use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use clap::{Arg, App};
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
mod terminal_adm3a;

#[cfg(windows)]
mod console_windows;
#[cfg(unix)]
mod console_unix;

use self::bdos::Bdos;
use self::bios::Bios;
use self::constants::*;
use self::cpm_machine::CpmMachine;
use self::fcb::*;
use self::terminal::TerminalEmulator;
use self::terminal::Transparent;
use self::terminal_adm3a::Adm3aToAnsi;

// Welcome message
const WELCOME: &str =
"iz-cpm https://github.com/ivanizag/iz-cpm
CP/M 2.2 Emulation
Press ctrl-c ctrl-c Y to return to host";

static CCP_BINARY: &[u8] = include_bytes!("../third-party/bin/zcpr.bin");

fn main() {
    // Parse arguments
    let matches = App::new(WELCOME)
        .arg(Arg::with_name("CMD")
            .help("The binary image to run, usually a .COM file")
            .required(false)
            .index(1))
            .arg(Arg::with_name("ARGS")
            .help("Parameters for the given command")
            .required(false)
            .index(2))
        .arg(Arg::with_name("call_trace")
            .short("t")
            .long("call-trace")
            .help("Traces BDOS calls excluding screen I/O"))
        .arg(Arg::with_name("call_trace_all")
            .short("T")
            .long("call-trace-all")
            .help("Traces BDOS and BIOS calls"))
        .arg(Arg::with_name("cpu_trace")
            .short("z")
            .long("cpu-trace")
            .help("Traces CPU instructions execution"))
        .arg(Arg::with_name("slow")
            .short("s")
            .long("slow")
            .help("Runs slower"))
        .arg(Arg::with_name("cpu")
            .long("cpu")
            .value_name("model")
            .default_value("z80")
            .help("Cpu model z80 or 8080"))
        .arg(Arg::with_name("terminal")
            .long("terminal")
            .default_value("adm3a")
            .help("Terminal emulation ADM-3A or ANSI"))
        .arg(Arg::with_name("ccp")
            .long("ccp")
            .value_name("ccp")
            .help("Alternative CPP bynary, it must be compiled with CCP_BASE=$f000"))
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
    let call_trace_all = matches.is_present("call_trace_all");
    let slow = matches.is_present("slow");
    let cpu_model = matches.value_of("cpu");
    let terminal = matches.value_of("terminal");
    let ccp_filename = matches.value_of("ccp");
    let use_tpa = filename.is_none();

    // Init device
    let mut machine = CpmMachine::new();
    let mut cpu = match cpu_model {
        Some("z80") => Cpu::new_z80(),
        Some("8080") => Cpu::new_8080(),
        _ => {
            eprintln!("Invalid CPU model. Choose \"z80\" or \"8080\" as the CPU.");
            return;
        }
    };
    let term_emu: Box<dyn TerminalEmulator> = match terminal {
        Some("adm3a") => Box::new(Adm3aToAnsi::new()),
        Some("ansi") => Box::new(Transparent::new()),
        _ => {
            eprintln!("Unkown terminal emulattion. Choose \"adm3a\" or \"ansi\".");
            return;
        }
    };

    // Init cpm
    let mut bios = Bios::new(term_emu);
    bios.setup(&mut machine);
    let mut bdos = Bdos::new();
    bdos.reset(&mut machine);

    // Assign drives
    for i in 0..15 {
        let res = matches.value_of(format!("disk_{}", (i + b'a') as char));
        if let Some(path) = res {
            if let Err(err) = fs::read_dir(path) {
                eprintln!("Error with directory \"{}\": {}", path, err);
                return;
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
            binary = match ccp_filename {
                None => {
                    binary_size = CCP_BINARY.len();
                    CCP_BINARY
                },
                Some(name) =>{
                    match File::open(name) {
                        Err(err) => {
                            eprintln!("Error opening ccp \"{}\": {}", name, err);
                            return; //process::exit(1);
                        },
                        Ok(mut file) => {
                            match file.read(&mut buf) {
                                Err(err) => {
                                    eprintln!("Error loading ccp \"{}\": {}", name, err);
                                    return; //process::exit(1);
                                },
                                Ok(size) => {
                                    binary_size = size;
                                    &buf
                                }
                            }
                        }
                    }
                }
            };
            binary_address = CCP_BASE_ADDRESS;
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
                    return; //process::exit(1);
                },
                Ok(mut file) => {
                    match file.read(&mut buf) {
                        Err(err) => {
                            eprintln!("Error loading \"{}\": {}", name, err);
                            return; //process::exit(1);
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

    // Load the code in memory
    for i in 0..binary_size {
        machine.poke(binary_address + i as u16, binary[i]);
    }

    if !use_tpa {
        // Upon entry to a transient program, the CCP leaves the stack pointer
        // set to an eight-level stack area with the CCP return address pushed
        // onto the stack, leaving seven levels before overflow occurs. 
        if binary_address == TPA_BASE_ADDRESS {
            let mut sp = TPA_STACK_ADDRESS;
            // Push 0x0000
            machine.poke(sp, (0x0000 >> 8) as u8);
            sp -= 1;
            machine.poke(sp, 0x0000_u8);
            sp -= 1;
            cpu.registers().set16(Reg16::SP, sp);
        }

        // Copy parameters As an added convenience, the default buffer area at
        // location BOOT+0080H is initialized to the command line tail typed by
        // the operator following the program name. The first position contains
        // the number of characters, with the characters themselves following
        // the character count. The characters are translated to upper-case
        // ASCII with uninitialized memory following the last valid character. 
        Fcb::new(FCB1_ADDRESS).set_name_direct(&mut machine, "        .   ".to_string());
        Fcb::new(FCB2_ADDRESS).set_name_direct(&mut machine, "        .   ".to_string());
        match params {
            None => machine.poke(SYSTEM_PARAMS_ADDRESS, 0),
            Some(p) => {
                let mut len = p.len();
                if len > 0x7E {
                    len = 0x7E; // Max 0x7E chars for parameters
                }
                machine.poke(SYSTEM_PARAMS_ADDRESS, (len + 1) as u8);
                machine.poke(SYSTEM_PARAMS_ADDRESS + 1, b' ');
                let p_bytes = p.as_bytes();
                for i in 0..len {
                    machine.poke(SYSTEM_PARAMS_ADDRESS + (i as u16) + 2, p_bytes[i]);
                }

                // As a convenience, the CCP takes the first two parameters that
                // appear in the command tail, attempts to parse them as though
                // they were file names, and places the results in FCBI and
                // FCB2. The results, in this context, mean that the logical
                // disk letter is converted to its FCB representation, and the
                // file name and type, converted to uppercase, are placed in the
                // FCB in the correct bytes. In addition, any use of "*" in the
                // file name is expanded to one or more question marks. For
                // example, a file name of "abc*.*" will be converted to a name
                // of "ABC!!???" and type of "???". Notice that FCB2 starts only
                // 16 bytes above FCBI, yet a normal FCB is at least 33 bytes
                // long (36 bytes if you want to use random access). In many
                // cases, programs only require a single file name. Therefore,
                // you can proceed to use FCBI straight away, not caring that
                // FCB2 will be overwritten.
                // Both are initialized with spaces.
                let mut parts = p.split_ascii_whitespace();
                if let Some(arg1) = parts.next() {
                    if let Some(file1) = name_to_8_3(arg1) {
                        if call_trace {
                            println!("[[FCB1 loaded with {}]]", file1);
                        }
                        Fcb::new(FCB1_ADDRESS).set_name_direct(&mut machine, file1);
                    }
                }
                if let Some(arg2) = parts.next() {
                    if let Some(file2) = name_to_8_3(arg2) {
                        if call_trace {
                            println!("[[FCB2 loaded with {}]]", file2);
                        }
                        Fcb::new(FCB2_ADDRESS).set_name_direct(&mut machine, file2);
                    }
                }
            }
        }
    }

    // Run the emulation
    cpu.registers().set_pc(binary_address);
    cpu.set_trace(cpu_trace);
    let mut n = 0;
    loop {
        cpu.execute_instruction(&mut machine);

        if cpu.is_halted() {
            println!("HALT instruction");
            break;
        }

        let mut er = bios.execute(cpu.registers(), call_trace_all);
        if er == ExecutionResult::Continue {
            er = bdos.execute(&mut bios, &mut machine, cpu.registers(),
                call_trace || call_trace_all, call_trace && ! call_trace_all);
        }
    
        match er {
            ExecutionResult::Continue => (),
            ExecutionResult::Stop => {
                break;
            },
            ExecutionResult::StopConfirm => {
                eprintln!();
                eprintln!("Press Y to exit iz-cpm. Any other key to continue.");
                let ch = bios.read() as char;
                if ch == 'Y' || ch == 'y' {
                    break;
                }
            },
            ExecutionResult::WarmBoot => {
                if call_trace || call_trace_all {
                    print!("[[Warm boot]]");
                }
                if use_tpa {
                    for i in 0..binary_size {
                        machine.poke(binary_address + i as u16, binary[i]);
                    }
                    cpu.registers().set_pc(binary_address);
                    let user_drive = machine.peek(CCP_USER_DRIVE_ADDRESS);
                    cpu.registers().set8(Reg8::C, user_drive);
                } else {
                    break;
                }
            },
            ExecutionResult::ColdBoot => {
                if call_trace || call_trace_all {
                    print!("[[Cold boot]]");
                }
                if use_tpa {
                    bdos.reset(&mut machine);
                    for i in 0..binary_size {
                        machine.poke(binary_address + i as u16, binary[i]);
                    }
                    cpu.registers().set_pc(binary_address);
                    cpu.registers().set8(Reg8::C, 0); // Reset user and drive
                    bdos.reset(&mut machine); // Reset Bdos
                } else {
                    break;
                }
            }
            

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