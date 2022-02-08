use iz80::Machine;

use super::bios::Bios;
use super::constants::*;
use super::cpm_machine::*;

pub const RECORD_SIZE: usize = 128;
pub const DEFAULT_DMA: u16 = 0x0080;

// Messages from http://www.gaby.de/cpm/manuals/archive/cpm22htm/axi.htm
pub const ERR_BAD_SECTOR: &str = "Bad Sector";
pub const ERR_DRIVE_READ_ONLY: &str = "R/O";

pub struct BdosState {
    pub user: u8,
    pub drive: u8,
    // Drive
    pub selected_bitmap: u16,
    pub read_only_bitmap: u16,
    pub directories: [Option<String>; 16],
    // File
    pub dma: u16,
    pub buffer: [u8; RECORD_SIZE],
    // DIR state
    pub dir_drive: u8,
    pub dir_pattern: String,
    pub dir_pos: u16, // We will hold a global position in a DIR.

}

impl BdosState {
    pub fn new() -> BdosState {
        BdosState {
            user: 0,
            drive: 0,
            selected_bitmap: 1<<0,
            read_only_bitmap: 0,
            directories: [None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None],
            dma: DEFAULT_DMA,
            dir_drive: 0,
            dir_pattern: "????????.???".to_string(),
            dir_pos: 0,
            buffer: [0; RECORD_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.user = 0;
        self.drive = 0;
        self.selected_bitmap = 1<<0;
        self.read_only_bitmap = 0;
        self.dma =  DEFAULT_DMA;
        self.dir_pattern = "????????.???".to_string();
        self.dir_pos = 0;
        self.buffer = [0; RECORD_SIZE];
    }
}

pub struct BdosEnvironment<'a> {
    pub state : &'a mut BdosState,
    pub bios: &'a mut Bios,
    pub machine: &'a mut CpmMachine,
    pub call_trace: bool
}

impl <'a> BdosEnvironment<'_> {
    pub fn new(
            state: &'a mut BdosState,
            bios: &'a mut Bios,
            machine: &'a mut CpmMachine,
            call_trace: bool) -> BdosEnvironment<'a> {
        BdosEnvironment {state, bios, machine, call_trace}
    }

    pub fn iobyte(&self) -> u8 {
        self.machine.peek(IOBYTE_ADDRESS) & 0x0f
    }
    pub fn set_iobyte(&mut self, iobyte: u8) {
        self.machine.poke(IOBYTE_ADDRESS, iobyte); 
    }

    pub fn load_buffer(&mut self) {
        for i in 0..RECORD_SIZE {
            self.machine.poke(self.state.dma + i as u16, self.state.buffer[i]);
        }
    }

    pub fn save_buffer(&mut self) {
        for i in 0..RECORD_SIZE {
            self.state.buffer[i] = self.machine.peek(self.state.dma + i as u16);
        }
    }

    pub fn get_directory(&self, fcb_drive: u8, to_write: bool) -> Option<String> {
        let drive = if fcb_drive == 0 {
            self.state.drive
        } else {
            fcb_drive - 1
        };

        if to_write && (self.state.read_only_bitmap & 1 << drive) != 0 {
            self.print_error_on_disk(ERR_DRIVE_READ_ONLY, drive);
            return None
        }

        let res = self.state.directories[drive as usize].clone();
        if res.is_none() {
            self.print_error_on_disk(ERR_BAD_SECTOR, drive);
        }
        res
    }

    pub fn print_error_on_disk(&self, message: &str, disk: u8) {
        println!("\nBdos Err On {}: {}", (b'A' + disk) as char, message);
    }
}
