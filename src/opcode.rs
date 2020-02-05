use std::num::Wrapping;

use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    pub bytes: usize,
    pub cycles: u64,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, bytes: usize, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, bytes, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })

    }
}

// ADD opcodes
pub fn build_add_hl_rr(p: usize) -> Opcode {
    let reg16 = &TABLE_RP[p];
    Opcode {
        name: format!("ADD HL, {}", TABLE_RP_NAME[p]),
        bytes: 1,
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = Wrapping(state.reg.get16(&Register16::HL));
            v = v + Wrapping(state.reg.get16(reg16));
            state.reg.set16(&Register16::HL, v.0); 
            // TODO: flags
        })
    }
}

// INC, DEC opcodes
pub fn build_inc_dec_rr(p: usize, inc: bool) -> Opcode {
    let reg16 = &TABLE_RP[p];
    let delta = if inc {1} else {65535};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {}", mnemonic, TABLE_RP_NAME[p]),
        bytes: 1,
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let mut v = Wrapping(state.reg.get16(reg16));
            v = v + Wrapping(delta);
            state.reg.set16(reg16, v.0);
            // Note: flags not affected
        })
    }    
}    

pub fn build_inc_r(y: usize) -> Opcode {
    let reg8 = &TABLE_R[y];
    Opcode {
        name: format!("INC {}", TABLE_R_NAME[y]),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(reg8);
            v = if v == 255 {0} else {v+1};

            state.reg.set8(reg8, v); 
            state.reg.update_sz53_flags(v);
            state.reg.clear_flag(&Flag::N);
            state.reg.put_flag(&Flag::P, v == 0x80);
            state.reg.put_flag(&Flag::H, (v & 0x0F) == 0x00);
            // Flag::C is not affected
        })
    }        
}

pub fn build_dec_r(y: usize) -> Opcode {
    let reg8 = &TABLE_R[y];
    Opcode {
        name: format!("DEC {}", TABLE_R_NAME[y]),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(reg8);
            v = if v == 0 {255} else {v-1};

            state.reg.set8(reg8, v); 
            state.reg.update_sz53_flags(v);
            state.reg.set_flag(&Flag::N);
            state.reg.put_flag(&Flag::P, v == 0x7F);
            state.reg.put_flag(&Flag::H, (v & 0x0F) == 0x0F);
            // Flag::C is not affected
        })
    }        
}

pub const TABLE_RP: [Register16; 4] = [
    Register16::BC, Register16::DE, Register16::HL, Register16::SP];
pub const TABLE_RP_NAME: [&str; 4] = [
    "BC", "DE", "HL", "SP"];
pub const TABLE_R:  [Register8; 8] = [
    Register8::B, Register8::C, Register8::D, Register8::E,
    Register8::H, Register8::L, Register8::_HL_, Register8::A];
pub const TABLE_R_NAME: [&str; 8] = [
    "B", "C", "D", "E",
    "H", "L", "undefined", "A"];

