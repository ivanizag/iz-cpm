use super::opcode::*;
use super::environment::*;
use super::registers::*;

// Relative jumps
pub fn build_djnz() -> Opcode {
    Opcode {
        name: "DJNZ d".to_string(),
        cycles: 8, // TODO: 13 jump,
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            let b = env.state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
            env.state.reg.set8(Reg8::B, b);
            if b != 0 {
                // Condition not met
                relative_jump(env, offset);
            }
        })
    }
}

pub fn build_jr_unconditional() -> Opcode {
    Opcode {
        name: "JR d".to_string(),
        cycles: 12,
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            relative_jump(env, offset);
        })
    }
}

pub fn build_jr_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JR {}, d", name),
        cycles: 7, // TODO: 12 jump,
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            if env.state.reg.get_flag(flag) == value {
                relative_jump(env, offset);
            }
        })
    }
}


fn relative_jump(env: &mut Environment, offset: u8) {
    let mut pc = env.state.reg.get_pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    pc = pc.wrapping_add(-2 as i16 as u16); // Assume rel jump opcode took 2 bytes
    env.state.reg.set_pc(pc);
}

// Absolute jumps
pub fn build_jp_unconditional() -> Opcode {
    Opcode {
        name: "JP d".to_string(),
        cycles: 10,
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.state.reg.set_pc(address);
        })
    }
}

pub fn build_jp_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JP {}, nn", name),
        cycles: 10, // TODO: 10 jump, review
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.state.reg.set_pc(address);
            }
        })
    }
}

pub fn build_jp_hl() -> Opcode {
    Opcode {
        name: "JP HL".to_string(), // Note: it is usaully written as JP (HL)
        cycles: 4, // IX/IY: 9
        action: Box::new(move |env: &mut Environment| {
            // Note: no displacement added to the index
            let address = env.get_index_value();
            env.state.reg.set_pc(address);
        })
    }
}

// Calls to subroutine
pub fn build_call() -> Opcode {
    Opcode {
        name: "CALL nn".to_string(),
        cycles: 10,
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.push(env.state.reg.get_pc());
            env.state.reg.set_pc(address);
        })
    }
}

pub fn build_call_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("CALL {}, nn", name),
        cycles: 10, // TODO: 17 calls,
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.push(env.state.reg.get_pc());
                env.state.reg.set_pc(address);
            }
        })
    }
}

pub fn build_rst(d: u8) -> Opcode {
    Opcode {
        name: format!("RST {:02x}h", d),
        cycles: 11,
        action: Box::new(move |env: &mut Environment| {
            let address = d as u16;
            env.push(env.state.reg.get_pc());
            env.state.reg.set_pc(address);
        })
    }
}

// Returns
fn operation_return(env: &mut Environment) {
    let pc = env.pop();
    env.state.reg.set_pc(pc);
}

pub fn build_ret() -> Opcode {
    Opcode {
        name: "RET".to_string(),
        cycles: 10,
        action: Box::new(move |env: &mut Environment| {
            operation_return(env);
        })
    }
}

pub fn build_reti() -> Opcode {
    Opcode {
        name: "RETI".to_string(),
        cycles: 14,
        action: Box::new(move |env: &mut Environment| {
            operation_return(env);
        })
    }
}

pub fn build_retn() -> Opcode {
    Opcode {
        name: "RETN".to_string(),
        cycles: 14,
        action: Box::new(move |env: &mut Environment| {
            operation_return(env);

            // TODO: "The contents of IIF2 is copied back into IIF1"
        })
    }
}

pub fn build_ret_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("RET {}", name),
        cycles: 5, // TODO: 11 returns,
        action: Box::new(move |env: &mut Environment| {
            if env.state.reg.get_flag(flag) == value {
                operation_return(env);
            }
        })
    }
}
