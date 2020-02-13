use super::opcode::*;
use super::state::*;
use super::registers::*;

// Relative jumps
pub fn build_djnz() -> Opcode {
    Opcode {
        name: "DJNZ d".to_string(),
        cycles: 8, // TODO: 13 jump,
        action: Box::new(move |state: &mut State| {
            let offset = state.advance_pc();
            let b = state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
            state.reg.set8(Reg8::B, b);
            if b != 0 {
                // Condition not met
                relative_jump(state, offset);
            }
        })
    }
}

pub fn build_jr_unconditional() -> Opcode {
    Opcode {
        name: "JR d".to_string(),
        cycles: 12,
        action: Box::new(move |state: &mut State| {
            let offset = state.advance_pc();
            relative_jump(state, offset);
        })
    }
}

pub fn build_jr_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JR {}, d", name),
        cycles: 7, // TODO: 12 jump,
        action: Box::new(move |state: &mut State| {
            let offset = state.advance_pc();
            if state.reg.get_flag(flag) == value {
                relative_jump(state, offset);
            }
        })
    }
}


fn relative_jump(state: &mut State, offset: u8) {
    let mut pc = state.reg.get_pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    pc = pc.wrapping_add(-2 as i16 as u16); // Assume rel jump opcode took 2 bytes
    state.reg.set_pc(pc);
}

// Absolute jumps
pub fn build_jp_unconditional() -> Opcode {
    Opcode {
        name: "JP d".to_string(),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            state.reg.set_pc(address);
        })
    }
}

pub fn build_jp_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JP {}, nn", name),
        cycles: 10, // TODO: 10 jump, review
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            if state.reg.get_flag(flag) == value {
                state.reg.set_pc(address);
            }
        })
    }
}

pub fn build_jp_hl() -> Opcode {
    Opcode {
        name: "JP (HL)".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::HL);
            state.reg.set_pc(address);
        })
    }
}

// Calls to subroutine
pub fn build_call() -> Opcode {
    Opcode {
        name: "CALL nn".to_string(),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            state.push(state.reg.get_pc());
            state.reg.set_pc(address);
        })
    }
}

pub fn build_call_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("CALL {}, nn", name),
        cycles: 10, // TODO: 17 calls,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            if state.reg.get_flag(flag) == value {
                state.push(state.reg.get_pc());
                state.reg.set_pc(address);
            }
        })
    }
}

pub fn build_rst(d: u8) -> Opcode {
    Opcode {
        name: format!("RST {:02x}h", d),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let address = d as u16;
            state.push(state.reg.get_pc());
            state.reg.set_pc(address);
        })
    }
}

// Returns
pub fn build_ret() -> Opcode {
    Opcode {
        name: "RET".to_string(),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let pc = state.pop();
            state.reg.set_pc(pc);
        })
    }
}

pub fn build_ret_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("RET {}", name),
        cycles: 5, // TODO: 11 returns,
        action: Box::new(move |state: &mut State| {
            if state.reg.get_flag(flag) == value {
                let pc = state.pop();
                state.reg.set_pc(pc);
            }
        })
    }
}
