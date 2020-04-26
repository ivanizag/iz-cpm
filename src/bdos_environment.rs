use iz80::Machine;

use super::bios::Bios;
use super::constants::*;
use super::cpm_machine::*;

pub const RECORD_SIZE: usize = 128;
pub const DEFAULT_DMA: u16 = 0x0080;

pub struct BdosState {
    // Drive
    pub selected_bitmap: u16,
    // File
    pub dma: u16,
    pub buffer: [u8; RECORD_SIZE],
    // DIR state
    pub dir_pattern: String,
    pub dir_pos: u16, // We will hold a global position in a DIR.

}

impl BdosState {
    pub fn new() -> BdosState {
        BdosState {
            selected_bitmap: 1<<0,
            dma: DEFAULT_DMA,
            dir_pattern: "????????.???".to_string(),
            dir_pos: 0,
            buffer: [0; RECORD_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.selected_bitmap = 1<<0;
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
        BdosEnvironment { state, bios, machine, call_trace}
    }

    pub fn user(&self) -> u8 {
        self.machine.peek(USER_DRIVE_ADDRESS) >> 4
    }
    pub fn set_user(&mut self, user: u8) {
        let current = self.machine.peek(USER_DRIVE_ADDRESS);
        self.machine.poke(USER_DRIVE_ADDRESS, (current & 0xf0) | (user << 4)); 
    }

    pub fn drive(&self) -> u8 {
        self.machine.peek(USER_DRIVE_ADDRESS) & 0x0f
    }
    pub fn set_drive(&mut self, drive: u8) {
        let current = self.machine.peek(USER_DRIVE_ADDRESS);
        self.machine.poke(USER_DRIVE_ADDRESS, (current & 0x0f) | drive); 
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
}
