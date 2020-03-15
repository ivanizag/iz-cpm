use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    pub cycles: u64,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }

    pub fn disasm(&self, state: &State) -> String {
        if self.name.contains("nn") {
            // Immediate argument 16 bits
            let nn = state.peek16_pc();
            let nn_str = format!("{:04x}h", nn);
            self.name.replace("nn", &nn_str)
        } else if self.name.contains("n") {
            // Immediate argument 8 bits
            let n = state.peek_pc();
            let n_str = format!("{:02x}h", n);
            self.name.replace("n", &n_str)
        } else if self.name.contains("n") {
            // Immediate argument 8 bits signed
            let d = state.peek_pc() as i8;
            let d_str = format!("{}", d);
            self.name.replace("d", &d_str)
        } else {
            self.name.clone()
        }
    }
}

pub fn operation_block(state: &mut State, inc: bool, repeat: bool, wide_b: bool) {
    if inc {
        state.reg.set16(Reg16::HL, state.reg.get16(Reg16::HL).wrapping_add(1));
    } else {
        state.reg.set16(Reg16::HL, state.reg.get16(Reg16::HL).wrapping_sub(1));
    }

    let repeat_cond: bool;
    if wide_b { // LDxx and CPxx
        let bc = state.reg.get16(Reg16::BC).wrapping_sub(1);
        state.reg.set16(Reg16::BC, bc);
        state.reg.put_flag(Flag::P, bc == 0);
        repeat_cond = bc != 0;
    } else { // INxx and OUTxx
        let b = state.reg.get8(Reg8::B).wrapping_sub(1);
        state.reg.set8(Reg8::B, b);
        state.reg.put_flag(Flag::Z, b == 0);
        repeat_cond = b != 0;
    }

    if repeat && repeat_cond {
        // Back to redo the instruction
        let pc = state.reg.get_pc().wrapping_sub(2);
        state.reg.set_pc(pc);
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })
    }
}

pub fn build_noni_nop() -> Opcode {
    Opcode {
        name: "NONINOP".to_string(),
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })
    }
}

pub fn build_halt() -> Opcode {
    Opcode {
        name: "HALT".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.halted = true;
        })
    }
}

pub fn build_pop_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("POP {:?}", rr),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.pop();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_push_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("PUSH {:?}", rr),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get16(rr);
            state.push(value);
        })
    }
}

pub fn build_conf_interrupts(enable: bool) -> Opcode {
    let name = if enable {"EI"} else  {"DI"};
    Opcode {
        name: name.to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.set_interrupts(enable);
        })
    }
}

pub fn build_im(im: u8) -> Opcode {
    Opcode {
        name: format!("IM {}", im),
        cycles: 8,
        action: Box::new(move |state: &mut State| {
            state.reg.set_interrup_mode(im);
        })
    }
}
