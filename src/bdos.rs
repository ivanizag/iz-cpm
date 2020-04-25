use iz80::*;

use super::bios::Bios;
use super::bdos_console::*;
use super::bdos_drive::*;
use super::bdos_file::*;
use super::cpm_machine::*;
use super::constants::*;
use super::fcb::Fcb;

pub struct Bdos {
    console: BdosConsole,
    drive: BdosDrive,
    file: BdosFile
}

const BDOS_COMMAND_NAMES: [&'static str; 38] = [
    "P_TERMCPM", "C_READ", "C_WRITE", "A_READ", "A_WRITE",
    "L_WRITE", "C_RAWIO", "A_STATIN", "A_STATOUT", "C_WRITESTR",
    "C_READSTR", "C_STAT", "S_BDOSVER", "DRV_ALLRESET", "DRV_SET",
    "F_OPEN", "F_CLOSE", "F_SFIRST", "F_SNEXT", "F_DELETE",
    "F_READ", "F_WRITE", "F_MAKE", "F_RENAME", "DRV_LOGINVEC",
    "DRV_GET", "F_DMAOFF", "DRV_ALLOCVEC", "DRV_SETRO", "DRV_ROVEC",
    "F_ATTRIB", "DRV_DPB", "F_USERNUM", "F_READRAND", "F_WRITERAND",
    "F_SIZE", "F_RANDREC", "DRV_RESET"]; 

impl Bdos {
    pub fn new() -> Bdos {
        Bdos {
            console: BdosConsole::new(),
            drive: BdosDrive::new(),
            file: BdosFile::new()
        }
    }

    pub fn setup(&self, machine: &mut CpmMachine) {
        /*
        Setup BDOS location and entry point
        .org $5
            jp BDOS_BASE_ADDRESS
        */
        machine.poke(5, 0xc3 /* jp nnnn */);
        machine.poke16(6, BDOS_BASE_ADDRESS);
        // We put ret on that address
        machine.poke(BDOS_BASE_ADDRESS, 0xc9 /*ret*/);
        /*
        Note: if the first 6 bytes of BDOS change, the serial number in the CCP
        source code needs to be updated.
        */
    }

    pub fn execute(&mut self, bios: &mut Bios,
            machine: &mut CpmMachine, reg: &mut Registers,
            call_trace: bool, call_trace_skip_console:bool) {

        // We do the BIOS actions outside the emulation.
        let pc = reg.pc();
        if pc == BDOS_BASE_ADDRESS {
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
                1=> { // C_READ - Console input
                    res8 = Some(self.console.read(bios))
                }
                2 => { // C_WRITE - Console output
                    self.console.write(bios,arg8);
                },
                6 => { // C_RAWIO - Direct console I/O
                    res8 = Some(self.console.raw_io(bios, arg8))
                }
                9 => { // C_WRITESTR - Output string
                    self.console.write_string(bios, &machine, arg16);
                },
                10 => { // C_READSTR
                    self.console.read_string(bios, machine, arg16);
                },
                11 => { // C_STAT - Console status
                    res8 = Some(self.console.status(bios));
                },
                12 => { // S_BDOSVER - Return version number
                    res16 = Some(get_version());
                },
                13 => { // DRV_ALLRESET - Reset disk system
                    self.drive.reset();
                    self.file.reset();
                },
                14 => { // DRV_SET - Select disk
                    self.drive.select(arg8);
                },
                15 => { // F_OPEN - Open file
                    let mut fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[[Open file {}]]", fcb.get_name());
                    }
                    res8 = Some(self.file.open(&mut fcb));
                },
                16 => { // F_CLOSE - Close file
                    let fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[[Close file {}]]", fcb.get_name());
                    }
                    res8 = Some(self.file.close(&fcb));
                },
                17 => { // F_SFIRST - Search for first
                    let fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[[DIR start {}]]", fcb.get_name());
                    }
                    let res = self.file.search_first(&fcb);
                    if res == 0 {
                        self.file.load_buffer(machine);
                    }
                    res8 = Some(res);

                }
                18 => { // F_SNEXT - Search for first
                    if call_trace {
                        print!("[[DIR next]]");
                    }
                    let res = self.file.search_next();
                    if res == 0 {
                        self.file.load_buffer(machine);
                    }
                    res8 = Some(res);
                }
                19 => { // F_DELETE - Delete file
                    let mut fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[[Delete file {}]]", fcb.get_name());
                    }
                    res8 = Some(self.file.delete(&mut fcb));
                }
                20 => { // F_READ - Read next record
                    let mut fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[Read record {:x} into {:04x}]",
                            fcb.get_sequential_record_number(), self.file.get_dma());
                    }
                    let res = self.file.read(&mut fcb);
                    if res == 0 {
                        self.file.load_buffer(machine);
                    }
                    res8 = Some(res);
                },
                21 => { // F_WRITE - Write next record
                    self.file.save_buffer(machine);
                    let mut fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[Write record {:x} from {:04x}]",
                            fcb.get_sequential_record_number(), self.file.get_dma());
                    }
                    let res = self.file.write(&mut fcb);
                    res8 = Some(res);
                }
                22 => { // F_MAKE - Create file
                    let mut fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[[Create file {}]]", fcb.get_name());
                    }
                    res8 = Some(self.file.make(&mut fcb));
                }
                24 => { // DRV_LOGINVEC - Return Log-in Vector
                    res16 = Some(self.drive.get_log_in_vector());
                },
                25 => { // DRV_GET - Return current disk
                    res8 = Some(self.drive.get_current());
                },
                26 => { // F_DMAOFF - Set DMA address
                    self.file.set_dma(arg16);
                },
                32 => { // F_USERNUM - Get/set user number
                    res8 = Some(self.file.get_set_user_number(arg8));
                }
                33 => { // F_READRAND - Random access read record
                    let fcb = Fcb::new(arg16, machine);
                    if call_trace {
                        print!("[Read random record {:x} into {:04x}]",
                            fcb.get_random_record_number(), self.file.get_dma());
                    }
                    let res = self.file.read_rand(&fcb);
                    if res == 0 {
                        self.file.load_buffer(machine);
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
                reg.set8(Reg8::A, a);
                reg.set8(Reg8::L, a);
                if bdos_trace {
                    println!("[[=>{:02x}]]", a);
                }
            } else if let Some(hl) = res16 {
                reg.set16(Reg16::HL, hl);
                reg.set8(Reg8::A, hl as u8);
                reg.set8(Reg8::B, (hl>>8) as u8);
                if bdos_trace {
                    println!("[[=>{:02x}]]", hl);
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