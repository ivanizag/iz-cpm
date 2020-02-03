use std::num::Wrapping;

use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    bytes: usize,
    cycles: u64,
    action: Box<OpcodeFn>,
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

pub fn build_ld_r_n(y: usize) -> Opcode {
    let reg8 = &TABLE_R[y];
    Opcode {
        name: format!("LD {}, X", TABLE_R_NAME[y]),
        bytes: 1,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            state.reg.set8(reg8, value);
            // TODO: flags
        })
    }
}

pub fn build_ld_rr_nn(p: usize) -> Opcode {
    let reg16 = &TABLE_RP[p];
    Opcode {
        name: format!("LD {}, XX", TABLE_RP_NAME[p]),
        bytes: 1,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(reg16, value);
            // TODO: flags
        })
    }
}

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
            // TODO: flags
        })
    }    
}    

pub fn build_inc_dec_r(y: usize, inc: bool) -> Opcode {
    let reg8 = &TABLE_R[y];
    let delta = if inc {1} else {255};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {}", mnemonic, TABLE_R_NAME[y]),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = Wrapping(state.reg.get8(reg8));
            v = v + Wrapping(delta);
            state.reg.set8(reg8, v.0); 
            // TODO: flags
        })
    }        
}


#[derive(Debug)]
struct DecodingHelper {
    // See notation in http://www.z80.info/decoding.htm    
    x: usize,
    y: usize,
    z: usize,
    p: usize,
    q: usize
}

impl DecodingHelper {
    fn parts(code: u8) -> DecodingHelper {
        DecodingHelper {
            x: (code >> 6) as usize,
            y: ((code >> 3) & 7) as usize,
            z: (code & 7) as usize,
            p: ((code >> 4) & 3) as usize,
            q: ((code >> 3) & 1) as usize,
        }
    }
}

const TABLE_RP: [Register16; 4] = [
    Register16::BC, Register16::DE, Register16::HL, Register16::SP];
const TABLE_RP_NAME: [&str; 4] = [
    "BC", "DE", "HL", "SP"];
const TABLE_R:  [Register8; 8] = [
    Register8::B, Register8::C, Register8::D, Register8::E,
    Register8::H, Register8::L, Register8::_HL_, Register8::A];
const TABLE_R_NAME: [&str; 8] = [
    "B", "C", "D", "E",
    "H", "L", "undefined", "A"];

