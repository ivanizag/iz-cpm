use super::registers::*;

pub struct State {
    pub reg: Registers,
    pub cycles: u64,
    pub halted: bool,
    // Alternate index management
    pub index: Reg16, // Using HL, IX or IY
    pub displacement: i8, // Used for (IX+d) and (iY+d)
    pub displacement_loaded: bool, // TODO: remove
    pub index_changed: bool, // Use the index change for the next opcode, reset afterwards
}

impl State {
    pub fn new() -> State {
        State {
            reg: Registers::new(),
            cycles: 0,
            halted: false,
            index: Reg16::HL,
            displacement: 0,
            displacement_loaded: false,
            index_changed: false
        }
    }
}
