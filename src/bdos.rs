use iz80::*;

use crate::bios::Bios;
use crate::bdos_environment::*;
use crate::bdos_console;
use crate::bdos_drive;
use crate::bdos_file;
use crate::console_emulator::ConsoleEmulator;
use crate::cpm_machine::CpmMachine;
use crate::constants::*;

const BDOS_COMMAND_NAMES: [&str; 106] = [
    // 0
    "P_TERMCPM", "C_READ", "C_WRITE", "A_READ", "A_WRITE",
    "L_WRITE", "C_RAWIO", "A_STATIN", "A_STATOUT", "C_WRITESTR",
    // 10
    "C_READSTR", "C_STAT", "S_BDOSVER", "DRV_ALLRESET", "DRV_SET",
    "F_OPEN", "F_CLOSE", "F_SFIRST", "F_SNEXT", "F_DELETE",
    // 20
    "F_READ", "F_WRITE", "F_MAKE", "F_RENAME", "DRV_LOGINVEC",
    "DRV_GET", "F_DMAOFF", "DRV_ALLOCVEC", "DRV_SETRO", "DRV_ROVEC",
    // 30
    "F_ATTRIB", "DRV_DPB", "F_USERNUM", "F_READRAND", "F_WRITERAND",
    "F_SIZE", "F_RANDREC", "DRV_RESET", "*", "",
    // 40
    "F_WRITEZ", "", "", "", "",
    "F_ERRMODE", "", "", "", "",

    "", "", "", "", "", "", "", "", "", "", // 50-59
    "", "", "", "", "", "", "", "", "", "", // 60-69
    "", "", "", "", "", "", "", "", "", "", // 70-79
    "", "", "", "", "", "", "", "", "", "", // 80-89
    "", "", "", "", "", "", "", "", "", "", // 00-09

    // 100
    "", "", "", "", "",
    "T_GET"
    ];

pub struct Bdos {
    state: BdosState,
}

impl Bdos {

    pub fn new() -> Bdos {
        Bdos {
            state: BdosState::new()
        }
    }

    pub fn warm_reset(&mut self, machine: &mut CpmMachine) {
        // Setup/Restore BOOT entrypoint
        machine.poke(  BDOS_ENTRY_ADDRESS,   0xc3 /* jp BDOS_BASE_ADDRESS */);
        machine.poke16(BDOS_ENTRY_ADDRESS+1, BDOS_BASE_ADDRESS);
    }

    pub fn reset(&mut self, machine: &mut CpmMachine) {
        self.state.reset();
        self.warm_reset(machine);

        // Reset IOBYTE
        machine.poke(IOBYTE_ADDRESS, 0);

        // We will trap here to execute the BDOS and then copy the result from
        // HL to A and B. It is done as code to make sure that the flags are
        // set correctly.
        // The actual CP/M 2.2 code executes these three instructions to return.
        machine.poke(BDOS_BASE_ADDRESS, 0x7d /*ld a,l*/);
        machine.poke(BDOS_BASE_ADDRESS+1, 0x44 /*ld b,h*/);
        machine.poke(BDOS_BASE_ADDRESS+2, 0xc9 /*ret*/);

        // Note: if the first 6 bytes of BDOS change, the serial number in the
        // CCP source code needs to be updated.

        // Disk parameter block 0
        // See "Programmer CP/M Handbook" by Andy Johnson-Laird, page 33
        machine.poke16(BDOS_DPB0_ADDRESS     ,  26);        // 128 bytes sectors per track
        machine.poke  (BDOS_DPB0_ADDRESS +  2,   3);        // Block shift for 1024 bytes block
        machine.poke  (BDOS_DPB0_ADDRESS +  3,   7);        // Block mask for 1024 bytes block
        machine.poke  (BDOS_DPB0_ADDRESS +  4,   3);        // Extent mask for 1024 bytes block
        machine.poke16(BDOS_DPB0_ADDRESS +  5, 242);        // Max allocation block number
        machine.poke16(BDOS_DPB0_ADDRESS +  7,  63);        // Number of directory entries - 1
        machine.poke  (BDOS_DPB0_ADDRESS +  9, 0b11000000); // Bitmap for allocation blocks
        machine.poke  (BDOS_DPB0_ADDRESS + 10, 0b00000000); // Bitmap for allocation blocks
        machine.poke16(BDOS_DPB0_ADDRESS + 11,  16);        // Max allocation block number
        machine.poke16(BDOS_DPB0_ADDRESS + 13,   2);        // Number of tracks before directory

        // Allocation vector 0, we need 30 bytes for 242 blocks
        for i in 0..30 {
            machine.poke(BDOS_ALVEC0_ADDRESS + i, 0);
        }
    }

    pub fn assign_drive(&mut self, drive: u8, path: String) {
        self.state.directories[(drive & 0x0f) as usize] = Some(path);
    }
}

pub fn execute_bdos(bdos: &mut Bdos, bios: &mut Bios, console: &mut dyn ConsoleEmulator,
        machine: &mut CpmMachine, reg: &mut Registers,
        call_trace: bool, call_trace_skip_console: bool) -> ExecutionResult {

    // We do the BIOS actions outside the emulation.
    let pc = reg.pc();
    if pc == BDOS_BASE_ADDRESS {
        let env = &mut BdosEnvironment::new(&mut bdos.state, bios, console, machine, call_trace);
        let arg8 = reg.get8(Reg8::E);
        let arg16 = reg.get16(Reg16::DE);
        let command = reg.get8(Reg8::C);

        let bdos_trace = call_trace && !(call_trace_skip_console && command <= 12);
        if bdos_trace {
            let name = if command < BDOS_COMMAND_NAMES.len() as u8 {
                BDOS_COMMAND_NAMES[command as usize]
            } else {
                "unknown"
            };
            print!("[[BDOS command {}: {}({:04x})]]", command, name, arg16);
        }

        let mut res8: Option<u8> = None;
        let mut res16: Option<u16> = None;

        match command {
            // See https://www.seasip.info/Cpm/bdos.html
            // See http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch5.htm
            0 => { // P_TERM_CPM - System reset
                // The System Reset function returns control to the CP/M
                // operating system at the CCP level. The CCP reinitializes
                // the disk subsystem by selecting and logging-in disk drive
                // A. This function has exactly the same effect as a jump to
                // location BOOT.
                return ExecutionResult::ColdBoot;
            },
            1 => { // C_READ - Console input
                res8 = Some(bdos_console::read(env));
            },
            2 => { // C_WRITE - Console output
                bdos_console::write(env, arg8);
            },
            3 => { // A_READ - Reader input
                // Use the console as the reader
                res8 = Some(bdos_console::read_reader(env))
            },
            4 => { // A_WRITE - Punch output
                // Use the console as the punch
                bdos_console::write(env, arg8);
            },
            5 => { // L_WRITE - List output
                // Use the console as the list
                bdos_console::write(env, arg8);
            },
            6 => { // C_RAWIO - Direct console I/O
                res8 = Some(bdos_console::raw_io(env, arg8))
            },
            7 => { // A_STATIN - Get I/O Byte
                res8 = Some(env.iobyte());
            },
            8 => { // A_STATOUT - Set I/O Byte
                env.set_iobyte(arg8);
            },
            9 => { // C_WRITESTR - Output string
                bdos_console::write_string(env, arg16);
            },
            10 => { // C_READSTR
                let result = bdos_console::read_string(env, arg16);
                if result == ExecutionResult::Continue{
                    return result;
                }
            },
            11 => { // C_STAT - Console status
                res8 = Some(bdos_console::status(env));
            },
            12 => { // S_BDOSVER - Return version number
                res16 = Some(get_version());
            },
            13 => { // DRV_ALLRESET - Reset disk system
                res8 = Some(bdos_drive::all_reset(env));
            },
            14 => { // DRV_SET - Select disk
                bdos_drive::select(env, arg8);
            },
            15 => { // F_OPEN - Open file
                res8 = Some(bdos_file::open(env, arg16));
            },
            16 => { // F_CLOSE - Close file
                res8 = Some(bdos_file::close(env, arg16));
            },
            17 => { // F_SFIRST - Search for first
                res8 = Some(bdos_file::search_first(env, arg16));
            },
            18 => { // F_SNEXT - Search for first
                res8 = Some(bdos_file::search_next(env));
            },
            19 => { // F_DELETE - Delete file
                res8 = Some(bdos_file::delete(env, arg16));
            },
            20 => { // F_READ - Read next record
                res8 = Some(bdos_file::read(env, arg16));
            },
            21 => { // F_WRITE - Write next record
                res8 = Some(bdos_file::write(env, arg16));
            },
            22 => { // F_MAKE - Create file
                res8 = Some(bdos_file::make(env, arg16));
            },
            23 => { // F_RENAME - Rename file
                res8 = Some(bdos_file::rename(env, arg16));
            },
            24 => { // DRV_LOGINVEC - Return Log-in Vector
                res16 = Some(bdos_drive::get_log_in_vector(env));
            },
            25 => { // DRV_GET - Return current disk
                res8 = Some(bdos_drive::get_current(env));
            },
            26 => { // F_DMAOFF - Set DMA address
                bdos_file::set_dma(env, arg16);
            },
            27 => { // DRV_ALLOCVEC - Get disk allocation vector
                res16 = Some(bdos_drive::get_disk_allocation_vector(env));
            },
            28 => { // DRV_SETRO - Write protect disk
                bdos_drive::set_disk_read_only(env);
            }
            29 => { // DRV_ROVEC - Get read-only vector
                res16 = Some(bdos_drive::get_read_only_vector(env));
            },
            30 => { // F_ATTRIB - Set File Attributes
                res8 = Some(bdos_file::set_attributes(env, arg16))
            },
            31 => { // DRV_DPB - Get disk parameter block
                res16 = Some(bdos_drive::get_disk_parameter_block(env));
            },
            32 => { // F_USERNUM - Get/set user number
                res8 = Some(bdos_file::get_set_user_number(env, arg8));
            },
            33 => { // F_READRAND - Random access read record
                res8 = Some(bdos_file::read_rand(env, arg16));
            },
            34 => { // F_WRITERAND - Write random
                res8 = Some(bdos_file::write_rand(env, arg16));
            },
            35 => { // F_SIZE - Compute file size
                bdos_file::compute_file_size(env, arg16);
            },
            36 => { // F_RANDREC - Set random record
                bdos_file::set_random_record(env, arg16);
            },
            37 => { // DRV_RESET - Reset drive
                res8 = Some(bdos_drive::reset_drives(env, arg16));
            },
            40 => { // F_WRITEZ - Write random with zero fill
                res8 = Some(bdos_file::write_rand_zero_fill(env, arg16));
            },
            45 => { // F_ERRMODE - Set action on hardware error
                bdos_file::set_error_mode(env, arg8);
            },


            105 => { // T_GET - Get date and time
                // Not implemented
                // Ignored silently to run https://github.com/sblendorio/gorilla-cpm
            },

            _ => {
                eprintln!("BDOS command {} not implemented.\n", command);
                return ExecutionResult::Stop;
            }
        }

        // Single byte values are returned in register A, with double byte
        // values returned in HL, a zero value is returned when the function
        // number is out of range. We put the result in HL, the return code
        // will copy it to A and B.
        let res = if let Some(a) = res8 {
            if bdos_trace {
                println!("[[=>{:02x}]]", a);
            }
            a as u16
        } else if let Some(hl) = res16 {
            if bdos_trace {
                println!("[[=>{:04x}]]", hl);
            }
            hl
        } else {
            0
        };

        reg.set16(Reg16::HL, res);
    }
    ExecutionResult::Continue
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